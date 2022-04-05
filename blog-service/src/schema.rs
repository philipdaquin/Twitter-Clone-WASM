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
        user_id -> Int4,
        slug -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        title -> Varchar,
        description -> Varchar,
        body -> Text,
        featured_image -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        created_at -> Timestamp,
        first_name -> Varchar,
        last_name -> Varchar,
        username -> Varchar,
        location -> Nullable<Varchar>,
        email -> Varchar,
    }
}

joinable!(comments -> posts (post_id));
joinable!(comments -> users (user_id));
joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    users,
);
