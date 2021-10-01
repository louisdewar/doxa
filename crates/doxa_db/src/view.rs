use crate::schema::{game_participants, games, leaderboard, users};

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

table! {
    active_games (id) {
        id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(active_agents, leaderboard);
allow_tables_to_appear_in_same_query!(active_agents, users);
allow_tables_to_appear_in_same_query!(active_agents, games);
allow_tables_to_appear_in_same_query!(active_agents, game_participants);
allow_tables_to_appear_in_same_query!(active_agents, active_games);
allow_tables_to_appear_in_same_query!(active_games, game_participants);
allow_tables_to_appear_in_same_query!(active_games, games);
