use actix_web::{web, HttpResponse};
use doxa_auth::guard::AuthGuard;
use doxa_core::{
    actix_web,
    chrono::{DateTime, Utc},
    EndpointResult,
};
use doxa_db::PgPool;
use doxa_executor::{client::GameClient, event::StartEvent};
use serde::Serialize;

use crate::error::{
    GameNotFound, IncorrectEventFormatting, IncorrectEventOrdering, MissingStartEvent,
    UnknownEventType,
};

use super::Competition;

#[derive(Serialize, Debug)]
pub struct EventResponse {
    // Will automatically serialize using an ISO 8601 format (rfc 3339) which should be compatable
    // with Javascript.
    start_time: DateTime<Utc>,
    complete_time: Option<DateTime<Utc>>,
    events: Vec<serde_json::Value>,
}

/// The default route for `_events/{game_id}`.
pub async fn default_events_route<C: Competition + ?Sized>(
    pg_pool: web::Data<PgPool>,
    path: web::Path<i32>,
    user: Option<AuthGuard<()>>,
) -> EndpointResult {
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
        move || doxa_db::action::game::get_game_events(&conn, game_id)
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
            "payload": serde_json::json!({ "agents": start_payload.agents })
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
                "client" => {
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
                event_type => {
                    return Err(UnknownEventType {
                        event_type: event_type.to_string(),
                    }
                    .into())
                }
            };

            output_events.push(serde_json::json!({
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
