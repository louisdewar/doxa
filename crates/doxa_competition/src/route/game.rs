use doxa_auth::guard::AuthGuard;
use doxa_core::{actix_web::web, error::HttpResponse, EndpointResult};

use doxa_db::model::game::Game;
use doxa_executor::{
    client::GameClient,
    event::{ErrorEvent, ForfeitEvent, StartEvent},
};
use serde_json::json;

use crate::{
    client::{Competition, Context},
    error::{GameNotFound, IncorrectEventFormatting, UnknownEventType},
};

use serde::Deserialize;

use super::response::{
    GameEventResponse, GameEventsResponse, GameResponse, GameResultResponse, PlayersResponse,
    PlayersResponsePlayer,
};

#[derive(Deserialize)]
pub struct GameEventsParams {
    #[serde(rename = "t")]
    event_type: Option<String>,
}

/// The default route for `_game/{game_id}`.
pub async fn game<C: Competition + ?Sized>(
    path: web::Path<i32>,
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let game_id = path.into_inner();

    let game = context
        .get_game_by_id(game_id)
        .await?
        .ok_or(GameNotFound { game_id })?;

    Ok(HttpResponse::Ok().json(GameResponse {
        game_id,
        queued_at: game.queued_at,
        started_at: game.started_at,
        completed_at: game.completed_at,
        outdated: game.outdated,
    }))
}

// TODO: get a single event endpoint
// Also maybe extract the logic for mapping events into either a trait or at the very least a function

/// The default route for `_game/{game_id}/events`.
pub async fn game_events<C: Competition + ?Sized>(
    path: web::Path<i32>,
    user: Option<AuthGuard<()>>,
    params: web::Query<GameEventsParams>,
    context: web::Data<Context<C>>,
) -> EndpointResult {
    // TODO: have competition have a method to return a CompetitionConfig which would allow things
    // such as making games entirely private by default to users who were not part of the
    // competition (not hugely important because of the filter but rn _START and _END will always
    // be returned).
    let game_id = path.into_inner();

    let _game: Game = context
        .get_game_by_id(game_id)
        .await?
        .ok_or(GameNotFound { game_id })?;

    let participants = context.get_game_participants_unordered(game_id).await?;

    let start_event = match context.get_start_event(game_id).await? {
        Some(event) => event,
        None => {
            return Ok(HttpResponse::Ok().json(GameEventsResponse { events: vec![] }));
        }
    };

    let (is_admin, agent_id) = user
            .map(|user| {
                let agent_id = participants.iter().find(|p| p.user == user.id()).map(|p| {
                    // The current user was a participant in the game, we are now finding their
                    // agent ID in game which is equal to the position within the Vec.
                    start_event.payload
                        .agents
                        .iter()
                        .position(|agent| agent == &p.agent)
                        .expect("agent was in the participant list but not in the list of agents in the start message")
                });

                (user.admin(), agent_id)
            })
            .unwrap_or((false, None));

    let events = if let Some(event_type) = params.event_type.clone() {
        context
            .get_game_events_by_event_type(game_id, event_type)
            .await?
    } else {
        context.get_events(game_id).await?
    };

    let mut output_events = Vec::with_capacity(events.len());
    for mut event in events {
        let event_id = event.event_id;
        let event = match event.event_type.as_str() {
            "_START" => {
                let payload: StartEvent = serde_json::from_value(event.payload).map_err(|e| {
                    IncorrectEventFormatting {
                        source: e,
                        event_id,
                    }
                })?;

                event.payload = json!({ "agents": payload.agents });

                event
            }
            "_END" => {
                // We don't want to leak the internal payload, if there is information we want
                // to send to the client we need to manually add it.
                event.payload = serde_json::Value::Null;

                event
            }
            "_FORFEIT" => {
                let payload: ForfeitEvent = serde_json::from_value(event.payload).map_err(|e| {
                    IncorrectEventFormatting {
                        source: e,
                        event_id,
                    }
                })?;

                // Admins or the owner of the agent
                if is_admin || agent_id == Some(payload.agent_id) {
                    event.payload = json!({ "agent": payload.agent_id, "stderr": payload.stderr, "reason": payload.error_message });
                } else {
                    event.payload = json!({ "agent": payload.agent_id });
                }

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
                    event.payload = json!({ "error": payload.error, "debug": payload.debug, "vm_logs": payload.vm_logs });
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

        output_events.push(GameEventResponse {
            id: event.event_id,
            event_type: event.event_type,
            timestamp: event.event_timestamp,
            payload: event.payload,
        });
    }

    Ok(HttpResponse::Ok().json(GameEventsResponse {
        events: output_events,
    }))
}

/// The default route for `_game/{game_id}/players`.
pub async fn game_players<C: Competition + ?Sized>(
    path: web::Path<i32>,
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let game_id = path.into_inner();

    if context.get_game_by_id(game_id).await?.is_none() {
        return Err(GameNotFound { game_id }.into());
    }

    let players = context
        .get_game_participants_ordered(game_id)
        .await?
        .into_iter()
        .map(|(agent, player)| PlayersResponsePlayer {
            username: player.username,
            agent,
        })
        .collect();

    Ok(HttpResponse::Ok().json(PlayersResponse { players }))
}

/// The default route for `_game/{game_id}/result/{agent}`.
pub async fn game_result_agent<C: Competition + ?Sized>(
    path: web::Path<(i32, String)>,
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let (game_id, agent_id) = path.into_inner();

    if context.get_game_by_id(game_id).await?.is_none() {
        return Err(GameNotFound { game_id }.into());
    }

    let result = context
        .get_game_result(game_id, agent_id)
        .await?
        .map(|result| result.result);

    Ok(HttpResponse::Ok().json(GameResultResponse { result }))
}
