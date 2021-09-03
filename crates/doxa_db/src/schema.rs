table! {
    agents (id) {
        id -> Text,
        owner -> Int4,
        competition -> Int4,
        extension -> Text,
        uploaded_at -> Timestamp,
        uploaded -> Bool,
        deleted -> Bool,
        failed -> Bool,
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
        event_id -> Int4,
        game -> Int4,
        event_timestamp -> Timestamp,
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
    games (id) {
        id -> Int4,
        start_time -> Timestamp,
        complete_time -> Nullable<Timestamp>,
        competition -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        admin -> Bool,
        username -> Text,
        password -> Text,
    }
}

joinable!(agents -> competitions (competition));
joinable!(agents -> users (owner));
joinable!(enrollment -> competitions (competition));
joinable!(enrollment -> users (user_id));
joinable!(game_events -> games (game));
joinable!(game_participants -> agents (agent));
joinable!(game_participants -> games (game));
joinable!(games -> competitions (competition));

allow_tables_to_appear_in_same_query!(
    agents,
    competitions,
    enrollment,
    game_events,
    game_participants,
    games,
    users,
);
