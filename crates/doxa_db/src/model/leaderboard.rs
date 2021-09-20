use crate::schema::leaderboard;

use diesel::{Insertable, Queryable};

#[derive(Debug, Clone, Insertable, Queryable)]
#[table_name = "leaderboard"]
pub struct LeaderboardScore {
    pub agent: String,
    pub score: i32,
}
