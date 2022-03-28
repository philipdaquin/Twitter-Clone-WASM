table! {
    posts (id) {
        id -> Uuid,
        author_id -> Uuid,
        slug -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        title -> Varchar,
        description -> Varchar,
        body -> Text,
        featured_image -> Text,
    }
}
