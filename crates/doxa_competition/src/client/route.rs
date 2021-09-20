use actix_web::{web, HttpResponse};
use doxa_auth::{error::UserNotFound, guard::AuthGuard};
use doxa_core::{
    actix_web,
    chrono::{DateTime, Utc},
    EndpointResult,
};
use doxa_db::PgPool;
use doxa_executor::{
    client::GameClient,
    event::{ErrorEvent, ForfeitEvent, StartEvent},
};
use serde::{Deserialize, Serialize};

use crate::error::{
    AgentNotFound, GameNotFound, IncorrectEventFormatting, IncorrectEventOrdering,
    MissingStartEvent, NoActiveAgent, UnknownEventType,
};

use serde_json::json;

use super::{Competition, Context};

// TODO:
// - Only let users view other user's data when they are enrolled in that competition
// - Check that a user is enrolled in this competition and return that as an error before returning
// the result for `_user` routes
// - Have default routes for `/_user/{username}` `/_game/{game_id}` etc..
// - split this file into separate routes for _user, _game, _leaderboard ...

#[derive(Deserialize)]
pub struct GameEventsParams {
    #[serde(rename = "t")]
    event_type: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct EventResponse {
    // Will automatically serialize using an ISO 8601 format (rfc 3339) which should be compatable
    // with Javascript.
    start_time: DateTime<Utc>,
    complete_time: Option<DateTime<Utc>>,
    events: Vec<serde_json::Value>,
}

/// The default route for `_game/{game_id}/events`.
pub async fn game_events<C: Competition + ?Sized>(
    pg_pool: web::Data<PgPool>,
    path: web::Path<i32>,
    user: Option<AuthGuard<()>>,
    params: web::Query<GameEventsParams>,
) -> EndpointResult {
    // TODO: switch to taking in Context to use a nicer API.

    // TODO: have competition have a method to return a CompetitionConfig which would allow things
    // such as making games entirely private by default to users who were not part of the
    // competition (not hugely important because of the filter but rn _START and _END will always
    // be returned).
    let game_id = path.into_inner();

    let game = web::block({
        let pg_pool = pg_pool.clone();
        let conn = web::block(move || pg_pool.get()).await??;
        move || doxa_db::action::game::get_game_by_id(&conn, game_id, C::COMPETITION_NAME)
    })
    .await??
    .ok_or(GameNotFound { game_id })?;

    let participants = web::block({
        let pg_pool = pg_pool.clone();
        let conn = web::block(move || pg_pool.get()).await??;
        move || doxa_db::action::game::get_game_participants(&conn, game_id)
    })
    .await??;

    let mut events = web::block({
        let pg_pool = pg_pool.clone();
        let conn = web::block(move || pg_pool.get()).await??;
        let event_type = params.event_type.clone();
        move || {
            if let Some(event_type) = event_type {
                doxa_db::action::game::get_game_events_by_event_type(&conn, game_id, event_type)
            } else {
                doxa_db::action::game::get_game_events(&conn, game_id)
            }
        }
    })
    .await??
    .into_iter()
    .peekable();

    // If there is at least one event then the first should have type _START:
    let events = if let Some(start_event) = events.next() {
        if &start_event.event_type != "_START" {
            return Err(MissingStartEvent {
                event_type: start_event.event_type.clone(),
            }
            .into());
        }

        let event_id = start_event.event_id;

        let start_payload: StartEvent =
            serde_json::from_value(start_event.payload).map_err(|e| IncorrectEventFormatting {
                source: e,
                event_id,
            })?;

        let (is_admin, agent_id) = user
            .map(|user| {
                let agent_id = participants.iter().find(|p| p.user == user.id()).map(|p| {
                    // The current user was a participant in the game, we are now finding their
                    // agent ID in game which is equal to the position within the Vec.
                    start_payload
                        .agents
                        .iter()
                        .position(|agent| agent == &p.agent)
                        .expect("agent was in the participant list but not in the list of agents in the start message")
                });

                (user.admin(), agent_id)
            })
            .unwrap_or((false, None));

        let mut output_events = Vec::with_capacity(events.len());

        output_events.push(serde_json::json!({
            "id": start_event.event_id,
            "type": start_event.event_type,
            "timestamp": start_event.event_timestamp,
            // Manually extract fields to avoid leaking
            "payload": json!({ "agents": start_payload.agents })
        }));

        while let Some(mut event) = events.next() {
            let event_id = event.event_id;
            let event = match event.event_type.as_str() {
                "_START" => return Err(IncorrectEventOrdering.into()),
                "_END" => {
                    // No events can occur after _END
                    if events.peek().is_some() {
                        return Err(IncorrectEventOrdering.into());
                    }

                    // We don't want to leak the internal payload, if there is information we want
                    // to send to the client we need to manually add it.
                    event.payload = serde_json::Value::Null;

                    event
                }
                "_FORFEIT" => {
                    let payload: ForfeitEvent =
                        serde_json::from_value(event.payload).map_err(|e| {
                            IncorrectEventFormatting {
                                source: e,
                                event_id,
                            }
                        })?;
                    event.payload = json!({ "agent": payload.agent_id });

                    event
                }
                "_ERROR" => {
                    // Admins get the full error
                    if is_admin {
                        let payload: ErrorEvent =
                            serde_json::from_value(event.payload).map_err(|e| {
                                IncorrectEventFormatting {
                                    source: e,
                                    event_id,
                                }
                            })?;
                        event.payload = json!({ "error": payload.error, "debug": payload.debug });
                    } else {
                        event.payload = serde_json::Value::Null;
                    }

                    event
                }
                event_type if !event_type.starts_with('_') => {
                    let payload: <C::GameClient as GameClient>::GameEvent =
                        serde_json::from_value(event.payload).map_err(|e| {
                            IncorrectEventFormatting {
                                source: e,
                                event_id,
                            }
                        })?;
                    if let Some(payload) = C::event_filter(payload, is_admin, agent_id) {
                        event.payload = payload;
                        event
                    } else {
                        // This event is being skipped
                        continue;
                    }
                }
                // Unknown system event
                event_type => {
                    return Err(UnknownEventType {
                        event_type: event_type.to_string(),
                    }
                    .into())
                }
            };

            output_events.push(json!({
                "id": event.event_id,
                "type": event.event_type,
                "timestamp": event.event_timestamp,
                "payload": event.payload
            }));
        }

        output_events
    } else {
        vec![]
    };

    Ok(HttpResponse::Ok().json(EventResponse {
        start_time: game.start_time,
        complete_time: game.complete_time,
        events,
    }))
}

/// The default route for `_agent/{agent_id}/games`.
pub async fn agent_games<C: Competition + ?Sized>(
    path: web::Path<String>,
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let agent_id = path.into_inner();

    // Used for ensuring that the agent exists otherwise the `get_agent_games` will return an
    // empty vec with no way to differentiate a non existant agent vs one with no matches
    let agent = context
        .get_agent(agent_id.clone())
        .await?
        .ok_or(AgentNotFound)?;

    let games: Vec<serde_json::Value> = context
        .get_agent_games(agent_id)
        .await?
        .into_iter()
        .map(|game| {
            json!({
                "id": game.id,
                "start_time": game.start_time,
                "end_time": game.complete_time
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(json!({ "agent": agent.id, "games": games })))
}

/// The default route for `_user/{username}/active_agent`.
pub async fn user_active_agent<C: Competition + ?Sized>(
    path: web::Path<String>,
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let username = path.into_inner();

    let user = context
        .get_user_by_username(username)
        .await?
        .ok_or(UserNotFound)?;

    // TODO: check user enrollment (also in other routes)

    let agent = context.get_active_agent(user.id).await?;

    // Either show the agent id or null if it is None:
    Ok(HttpResponse::Ok().json(json!({ "active_agent": agent.map(|agent| agent.id) })))
}

// TODO: this route may not be a good idea because historical scores are likely to be inaccurate
/// The default route for `_user/{username}/high_score`.
pub async fn user_high_score<C: Competition + ?Sized>(
    path: web::Path<String>,
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let username = path.into_inner();

    let user = context
        .get_user_by_username(username)
        .await?
        .ok_or(UserNotFound)?;

    let best = context.get_high_score(user.id).await?;

    let score = best.as_ref().map(|best| best.score);
    let agent = best.map(|best| best.agent);

    Ok(HttpResponse::Ok().json(json!({ "agent": agent, "score": score })))
}

/// The default route for `_user/{username}/score`.
pub async fn user_score<C: Competition + ?Sized>(
    path: web::Path<String>,
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let username = path.into_inner();

    let user = context
        .get_user_by_username(username)
        .await?
        .ok_or(UserNotFound)?;

    // TODO: check user enrollment (also in other routes)

    let agent = context
        .get_active_agent(user.id)
        .await?
        .ok_or(NoActiveAgent)?;

    let score = context.get_agent_score(agent.id.clone()).await?;

    Ok(HttpResponse::Ok().json(json!({ "agent": agent.id, "score": score })))
}

/// The default route for `_user/{username}/agents`.
pub async fn user_agents<C: Competition + ?Sized>(
    path: web::Path<String>,
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let username = path.into_inner();

    let user = context
        .get_user_by_username(username)
        .await?
        .ok_or(UserNotFound)?;

    // TODO: check user enrollment

    let agents = context
        .get_user_agents(user.id)
        .await?
        .into_iter()
        .map(|agent| json!({ "id": agent.id, "uploaded_at": agent.uploaded_at }))
        .collect::<Vec<_>>();

    // Either show the agent id or null if it is None:
    Ok(HttpResponse::Ok().json(json!({ "agents": agents })))
}

/// The default route for `_agent/{agent_id}/score`.
pub async fn agent_score<C: Competition + ?Sized>(
    path: web::Path<String>,
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let agent_id = path.into_inner();

    let agent = context
        .get_agent(agent_id.clone())
        .await?
        .ok_or(AgentNotFound)?;

    let score = context.get_agent_score(agent.id.clone()).await?;

    Ok(HttpResponse::Ok().json(json!({ "agent": agent.id, "score": score })))
}
