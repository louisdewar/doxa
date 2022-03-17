use diesel::JoinOnDsl;
use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};

use crate::model::leaderboard::{InsertableLeaderboardScore, LeaderboardScore};
use crate::model::storage::AgentUpload;
use crate::model::user::User;
use crate::schema as s;
use crate::view;
use crate::DieselError;

const DEFAULT_LEADERBOARD_KEY: &str = "primary";

/// Inserts a new scoreboard entry returning an error if there is already an agent in the
/// scoreboard
pub fn insert_new_score(
    conn: &PgConnection,
    key: Option<String>,
    agent: String,
    score: i32,
) -> Result<LeaderboardScore, DieselError> {
    diesel::insert_into(s::leaderboard::table)
        .values(InsertableLeaderboardScore { key, agent, score })
        .get_result(conn)
}

/// Either inserts a new score or overwrites the previous one for this agent
pub fn upsert_score(
    conn: &PgConnection,
    key: Option<String>,
    agent: String,
    score: i32,
) -> Result<LeaderboardScore, DieselError> {
    diesel::insert_into(s::leaderboard::table)
        .values(InsertableLeaderboardScore { key, agent, score })
        .on_conflict((s::leaderboard::columns::agent, s::leaderboard::columns::key))
        .do_update()
        .set(s::leaderboard::columns::score.eq(score))
        .get_result(conn)
}

/// Adds the given score to whatever the current score for the agent is.
/// If there is no current score then the default is inserted AND then the delta is added to it
/// such that final result is `default + delta`
pub fn update_score(
    conn: &PgConnection,
    key: Option<String>,
    agent: String,
    delta: i32,
    default: i32,
) -> Result<LeaderboardScore, DieselError> {
    diesel::insert_into(s::leaderboard::table)
        .values(InsertableLeaderboardScore {
            key,
            agent,
            score: delta + default,
        })
        .on_conflict((s::leaderboard::columns::agent, s::leaderboard::columns::key))
        .do_update()
        .set(s::leaderboard::columns::score.eq(s::leaderboard::columns::score + delta))
        .get_result(conn)
}

pub fn get_score(
    conn: &PgConnection,
    key: Option<String>,
    agent: String,
) -> Result<Option<LeaderboardScore>, DieselError> {
    let key = key.unwrap_or_else(|| DEFAULT_LEADERBOARD_KEY.to_string());
    s::leaderboard::table
        .filter(s::leaderboard::columns::agent.eq(agent))
        .filter(s::leaderboard::columns::key.eq(key))
        .first(conn)
        .optional()
}

pub fn get_user_high_score(
    conn: &PgConnection,
    user: i32,
    competition: i32,
    key: Option<String>,
) -> Result<Option<LeaderboardScore>, DieselError> {
    let key = key.unwrap_or_else(|| DEFAULT_LEADERBOARD_KEY.to_string());
    s::agents::table
        .filter(s::agents::owner.eq(user))
        .filter(s::agents::competition.eq(competition))
        .inner_join(s::leaderboard::table)
        .filter(s::leaderboard::columns::key.eq(key))
        .select(s::leaderboard::all_columns)
        .order_by(s::leaderboard::score.desc())
        .first(conn)
        .optional()
}

pub fn get_user_rank(
    _conn: &PgConnection,
    _user: i32,
) -> Result<Option<(i32, LeaderboardScore)>, DieselError> {
    todo!();
    // s::leaderboard::table
    //     .filter(s::leaderboard::columns::agent.eq(agent))
    //     .first(conn)
    //     .optional()
}

/// Returns the list of agents in order of score (descending) for all active agents within a particular competition
pub fn active_leaderboard(
    conn: &PgConnection,
    competition: i32,
    key: Option<String>,
) -> Result<Vec<(User, LeaderboardScore, AgentUpload)>, DieselError> {
    let key = key.unwrap_or_else(|| DEFAULT_LEADERBOARD_KEY.to_string());
    view::active_agents::table
        .filter(view::active_agents::competition.eq(competition))
        .inner_join(s::leaderboard::table.on(s::leaderboard::agent.eq(view::active_agents::id)))
        .filter(s::leaderboard::columns::key.eq(key))
        .inner_join(s::users::table.on(s::users::id.eq(view::active_agents::owner)))
        .order_by(s::leaderboard::score.desc())
        .order_by(view::active_agents::uploaded_at.desc())
        .select((
            s::users::all_columns,
            s::leaderboard::all_columns,
            view::active_agents::all_columns,
        ))
        .get_results(conn)
}
