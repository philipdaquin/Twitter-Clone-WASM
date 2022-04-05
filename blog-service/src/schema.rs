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

table! {
    user_comment (id) {
        id -> Int4,
        username -> Varchar,
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
        hash -> Varchar,
        role -> Varchar,
    }
}

table! {
    valid_roles (role) {
        role -> Varchar,
    }
}

joinable!(comments -> posts (post_id));
joinable!(comments -> user_comment (user_id));
joinable!(users -> valid_roles (role));

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    user_comment,
    users,
    valid_roles,
);
