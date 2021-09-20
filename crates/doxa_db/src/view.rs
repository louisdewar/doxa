use crate::schema::{agents, leaderboard};

table! {
    active_agents (id) {
        id -> Text,
        owner -> Int4,
        competition -> Int4,
        extension -> Text,
        uploaded_at -> Timestamptz,
        uploaded -> Bool,
        deleted -> Bool,
        failed -> Bool,
        active -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(active_agents, leaderboard);
