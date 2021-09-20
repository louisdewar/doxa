use crate::model::game as model;

use crate::{schema as s, DieselError};
use chrono::{DateTime, Utc};
use diesel::{dsl, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};

pub fn create_game(
    conn: &PgConnection,
    game: &model::InsertableGame,
) -> Result<model::Game, DieselError> {
    diesel::insert_into(s::games::table)
        .values(game)
        .get_result(conn)
}

pub fn set_game_complete_time(
    conn: &PgConnection,
    game_id: i32,
    complete_time: DateTime<Utc>,
) -> Result<model::Game, DieselError> {
    diesel::update(s::games::table)
        .filter(s::games::columns::id.eq(game_id))
        .set(s::games::columns::complete_time.eq(complete_time))
        .get_result(conn)
}

pub fn add_participant(
    conn: &PgConnection,
    participant: &model::GameParticipant,
) -> Result<model::GameParticipant, DieselError> {
    diesel::insert_into(s::game_participants::table)
        .values(participant)
        .get_result(conn)
}

pub fn add_event(
    conn: &PgConnection,
    event: &model::GameEvent,
) -> Result<model::GameEvent, DieselError> {
    diesel::insert_into(s::game_events::table)
        .values(event)
        .get_result(conn)
}

pub fn get_game_by_id(
    conn: &PgConnection,
    id: i32,
    competition_name: &str,
) -> Result<Option<model::Game>, DieselError> {
    s::games::table
        .inner_join(s::competitions::table)
        .filter(s::competitions::columns::name.eq(competition_name))
        .filter(s::games::columns::id.eq(id))
        .select(s::games::all_columns)
        .first(conn)
        .optional()
}

pub fn get_game_events(conn: &PgConnection, id: i32) -> Result<Vec<model::GameEvent>, DieselError> {
    s::game_events::table
        .filter(s::game_events::columns::game.eq(id))
        .order_by(s::game_events::columns::event_id.asc())
        .get_results(conn)
}

pub fn get_game_events_by_event_type(
    conn: &PgConnection,
    id: i32,
    event_type: String,
) -> Result<Vec<model::GameEvent>, DieselError> {
    s::game_events::table
        .filter(s::game_events::columns::game.eq(id))
        .filter(s::game_events::columns::event_type.eq(event_type))
        .order_by(s::game_events::columns::event_id.asc())
        .get_results(conn)
}

/// Return a single event by event type.
/// If there are more than one of this event type for a particular game then which event is returned is undefined.
pub fn get_single_game_event_by_event_type(
    conn: &PgConnection,
    id: i32,
    event_type: String,
) -> Result<Option<model::GameEvent>, DieselError> {
    s::game_events::table
        .filter(s::game_events::columns::game.eq(id))
        .filter(s::game_events::columns::event_type.eq(event_type))
        .first(conn)
        .optional()
}

pub fn get_game_participants(
    conn: &PgConnection,
    id: i32,
) -> Result<Vec<model::GameParticipantUser>, DieselError> {
    s::game_participants::table
        .filter(s::game_participants::game.eq(id))
        .inner_join(s::agents::table)
        .select((s::game_participants::agent, s::agents::owner))
        .get_results(conn)
}

pub fn get_agent_games(
    conn: &PgConnection,
    agent: String,
) -> Result<Vec<model::Game>, DieselError> {
    s::games::table
        .inner_join(s::game_participants::table)
        .filter(s::game_participants::columns::agent.eq(agent))
        .order_by(s::games::columns::start_time.asc())
        .select(s::games::all_columns)
        .get_results(conn)
}

pub fn add_game_result(
    conn: &PgConnection,
    result: &model::GameResult,
) -> Result<model::GameResult, DieselError> {
    diesel::insert_into(s::game_results::table)
        .values(result)
        .get_result(conn)
}

pub fn sum_game_results(conn: &PgConnection, agent: String) -> Result<Option<i64>, DieselError> {
    s::game_results::table
        .select(dsl::sum(s::game_results::result))
        .filter(s::game_results::agent.eq(agent))
        .first(conn)
}

pub fn remove_game_result_by_participant(
    conn: &PgConnection,
    agent: String,
) -> Result<Vec<model::GameResult>, DieselError> {
    let games = s::game_participants::table
        .filter(s::game_participants::agent.eq(agent))
        .select(s::game_participants::game);
    diesel::delete(s::game_results::table.filter(s::game_results::game.eq_any(games)))
        .get_results(conn)
}
