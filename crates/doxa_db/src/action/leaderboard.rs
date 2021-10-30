use diesel::JoinOnDsl;
use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};

use crate::model::leaderboard::LeaderboardScore;
use crate::model::user::User;
use crate::schema as s;
use crate::view;
use crate::DieselError;

/// Inserts a new scoreboard entry returning an error if there is already an agent in the
/// scoreboard
pub fn insert_new_score(
    conn: &PgConnection,
    agent: String,
    score: i32,
) -> Result<LeaderboardScore, DieselError> {
    diesel::insert_into(s::leaderboard::table)
        .values(LeaderboardScore { agent, score })
        .get_result(conn)
}

/// Either inserts a new score or overwrites the previous one for this agent
pub fn upsert_score(
    conn: &PgConnection,
    agent: String,
    score: i32,
) -> Result<LeaderboardScore, DieselError> {
    diesel::insert_into(s::leaderboard::table)
        .values(LeaderboardScore { agent, score })
        .on_conflict(s::leaderboard::columns::agent)
        .do_update()
        .set(s::leaderboard::columns::score.eq(score))
        .get_result(conn)
}

/// Adds the given score to whatever the current score for the agent is.
/// If there is no current score then the default is inserted AND then the delta is added to it
/// such that final result is `default + delta`
pub fn update_score(
    conn: &PgConnection,
    agent: String,
    delta: i32,
    default: i32,
) -> Result<LeaderboardScore, DieselError> {
    diesel::insert_into(s::leaderboard::table)
        .values(LeaderboardScore {
            agent,
            score: delta + default,
        })
        .on_conflict(s::leaderboard::columns::agent)
        .do_update()
        .set(s::leaderboard::columns::score.eq(s::leaderboard::columns::score + delta))
        .get_result(conn)
}

pub fn get_score(
    conn: &PgConnection,
    agent: String,
) -> Result<Option<LeaderboardScore>, DieselError> {
    s::leaderboard::table
        .filter(s::leaderboard::columns::agent.eq(agent))
        .first(conn)
        .optional()
}

pub fn get_user_high_score(
    conn: &PgConnection,
    user: i32,
) -> Result<Option<LeaderboardScore>, DieselError> {
    s::agents::table
        .filter(s::agents::owner.eq(user))
        .inner_join(s::leaderboard::table)
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
) -> Result<Vec<(User, LeaderboardScore)>, DieselError> {
    view::active_agents::table
        .filter(view::active_agents::competition.eq(competition))
        .inner_join(s::leaderboard::table.on(s::leaderboard::agent.eq(view::active_agents::id)))
        .inner_join(s::users::table.on(s::users::id.eq(view::active_agents::owner)))
        .order_by(s::leaderboard::score.desc())
        .select((s::users::all_columns, s::leaderboard::all_columns))
        .get_results(conn)
}
