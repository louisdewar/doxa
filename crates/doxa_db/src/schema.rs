table! {
    agents (id) {
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
    competitions (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    enrollment (user_id, competition) {
        user_id -> Int4,
        competition -> Int4,
    }
}

table! {
    game_events (event_id, game) {
        game -> Int4,
        event_id -> Int4,
        event_timestamp -> Timestamptz,
        event_type -> Text,
        payload -> Jsonb,
    }
}

table! {
    game_participants (agent, game) {
        agent -> Text,
        game -> Int4,
    }
}

table! {
    game_results (agent, game) {
        agent -> Text,
        game -> Int4,
        result -> Int4,
    }
}

table! {
    games (id) {
        id -> Int4,
        start_time -> Timestamptz,
        complete_time -> Nullable<Timestamptz>,
        competition -> Int4,
    }
}

table! {
    invites (id) {
        id -> Text,
        username -> Nullable<Text>,
        enrollments -> Array<Text>,
        expires_at -> Nullable<Timestamptz>,
    }
}

table! {
    leaderboard (agent) {
        agent -> Text,
        score -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        admin -> Bool,
        username -> Text,
        password -> Text,
        token_generation -> Text,
    }
}

joinable!(agents -> competitions (competition));
joinable!(agents -> users (owner));
joinable!(enrollment -> competitions (competition));
joinable!(enrollment -> users (user_id));
joinable!(game_events -> games (game));
joinable!(game_participants -> agents (agent));
joinable!(game_participants -> games (game));
joinable!(game_results -> agents (agent));
joinable!(game_results -> games (game));
joinable!(games -> competitions (competition));
joinable!(leaderboard -> agents (agent));

allow_tables_to_appear_in_same_query!(
    agents,
    competitions,
    enrollment,
    game_events,
    game_participants,
    game_results,
    games,
    invites,
    leaderboard,
    users,
);
