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

joinable!(users -> valid_roles (role));

allow_tables_to_appear_in_same_query!(
    users,
    valid_roles,
);
