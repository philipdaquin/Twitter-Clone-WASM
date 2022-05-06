table! {
    comments (id) {
        id -> Int4,
        author_id -> Int4,
        post_id -> Int4,
        body -> Varchar,
        media -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
