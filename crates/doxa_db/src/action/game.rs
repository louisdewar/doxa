use crate::model::game as model;

use crate::{schema as s, view, DieselError};
use chrono::{DateTime, Utc};
use diesel::{
    dsl, ExpressionMethods, JoinOnDsl, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl,
};

pub fn create_game(
    conn: &PgConnection,
    game: &model::InsertableGame,
) -> Result<model::Game, DieselError> {
    diesel::insert_into(s::games::table)
        .values(game)
        .get_result(conn)
}

pub fn set_game_start_time(
    conn: &PgConnection,
    game_id: i32,
    started_at: DateTime<Utc>,
) -> Result<model::Game, DieselError> {
    diesel::update(s::games::table)
        .filter(s::games::columns::id.eq(game_id))
        .set(s::games::columns::started_at.eq(started_at))
        .get_result(conn)
}

pub fn set_game_complete_time(
    conn: &PgConnection,
    game_id: i32,
    complete_at: DateTime<Utc>,
) -> Result<model::Game, DieselError> {
    diesel::update(s::games::table)
        .filter(s::games::columns::id.eq(game_id))
        .set(s::games::columns::completed_at.eq(complete_at))
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

pub fn get_game_by_id_required(
    conn: &PgConnection,
    id: i32,
    competition_name: &str,
) -> Result<model::Game, DieselError> {
    s::games::table
        .inner_join(s::competitions::table)
        .filter(s::competitions::columns::name.eq(competition_name))
        .filter(s::games::columns::id.eq(id))
        .select(s::games::all_columns)
        .first(conn)
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

pub fn get_game_participants_unordered(
    conn: &PgConnection,
    id: i32,
) -> Result<Vec<model::GameParticipantUser>, DieselError> {
    s::game_participants::table
        .filter(s::game_participants::game.eq(id))
        .inner_join(s::agents::table)
        .select((s::game_participants::agent, s::agents::owner))
        .get_results(conn)
}

pub fn get_game_participants_ordered(
    conn: &PgConnection,
    id: i32,
) -> Result<Vec<(String, crate::model::user::User)>, DieselError> {
    s::game_participants::table
        .filter(s::game_participants::game.eq(id))
        .order(s::game_participants::index)
        .inner_join(s::agents::table)
        .inner_join(s::users::table.on(s::users::id.eq(s::agents::owner)))
        .select((s::game_participants::agent, s::users::all_columns))
        .get_results(conn)
}

pub fn get_agent_games(
    conn: &PgConnection,
    agent: String,
) -> Result<Vec<model::Game>, DieselError> {
    s::games::table
        .inner_join(s::game_participants::table)
        .filter(s::game_participants::columns::agent.eq(agent))
        .order_by(s::games::columns::queued_at.asc())
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

pub fn get_game_result(
    conn: &PgConnection,
    game_id: i32,
    agent_id: String,
) -> Result<Option<model::GameResult>, DieselError> {
    s::game_results::table
        .filter(s::game_results::agent.eq(agent_id))
        .filter(s::game_results::game.eq(game_id))
        .get_result(conn)
        .optional()
}

/// Sums games results for a particular agent only from games where outdated = false
pub fn sum_non_outdated_game_results(
    conn: &PgConnection,
    agent: String,
) -> Result<Option<i64>, DieselError> {
    s::game_results::table
        .inner_join(s::games::table)
        .filter(s::games::columns::outdated.eq(false))
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

/// Get games that involve only active agents where the user has participated in order of the time
/// that they were queued_at (maybe change to only include started games and order by started_at)
pub fn get_user_active_games(
    conn: &PgConnection,
    user_id: i32,
    competition_id: i32,
) -> Result<Vec<model::Game>, DieselError> {
    view::active_agents::table
        .filter(view::active_agents::competition.eq(competition_id))
        .filter(view::active_agents::owner.eq(user_id))
        .inner_join(
            s::game_participants::table.on(s::game_participants::agent.eq(view::active_agents::id)),
        )
        .inner_join(
            view::active_games::table.on(view::active_games::id.eq(s::game_participants::game)),
        )
        .inner_join(s::games::table.on(s::games::id.eq(s::game_participants::game)))
        .order_by(s::games::columns::queued_at.asc())
        .select(s::games::all_columns)
        .get_results(conn)
}

/// Finds all the games that involve a paticular user and mark them as inactive.
/// This is done by player instead of agent to make it more resliant in the case of a crash when
/// activating / deactivating an agent
pub fn mark_games_with_player_as_outdated(
    conn: &PgConnection,
    user: i32,
    competition: i32,
) -> Result<(), DieselError> {
    use s::game_participants::columns as p_c;
    use s::games::columns as g_c;

    diesel::update(s::games::table)
        .filter(g_c::outdated.eq(false))
        .filter(g_c::competition.eq(competition))
        .filter(
            g_c::id.eq_any(
                s::game_participants::table
                    .inner_join(s::agents::table)
                    .filter(s::agents::owner.eq(user))
                    .select(p_c::game),
            ),
        )
        .set(g_c::outdated.eq(true))
        .execute(conn)
        .map(|_rows: usize| ())
}
