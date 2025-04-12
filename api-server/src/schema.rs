diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        created_at -> Timestamp,
    }
}
