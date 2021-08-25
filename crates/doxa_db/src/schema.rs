table! {
    agents (id) {
        id -> Text,
        owner -> Int4,
        competition -> Int4,
        extension -> Text,
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

allow_tables_to_appear_in_same_query!(
    agents,
    competitions,
    enrollment,
    users,
);
