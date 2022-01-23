use crate::schema::leaderboard;

use diesel::{Insertable, Queryable};

#[derive(Debug, Clone, Queryable)]
pub struct LeaderboardScore {
    pub key: String,
    pub agent: String,
    pub score: i32,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[table_name = "leaderboard"]
pub struct InsertableLeaderboardScore {
    pub key: Option<String>,
    pub agent: String,
    pub score: i32,
}
