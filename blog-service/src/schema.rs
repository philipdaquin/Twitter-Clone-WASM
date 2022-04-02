table! {
    comments (id) {
        id -> Int4,
        user_id -> Int4,
        post_id -> Int4,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    posts (id) {
        id -> Int4,
        author_id -> Int4,
        slug -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        title -> Varchar,
        description -> Varchar,
        body -> Text,
        featured_image -> Text,
    }
}

joinable!(comments -> posts (post_id));

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
);
