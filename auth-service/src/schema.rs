table! {
    authors (id) {
        id -> Int4,
        name -> Varchar,
        age -> Nullable<Int4>,
    }
}

table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        genre -> Varchar,
        user_id -> Int4,
    }
}

joinable!(books -> authors (user_id));

allow_tables_to_appear_in_same_query!(
    authors,
    books,
);
