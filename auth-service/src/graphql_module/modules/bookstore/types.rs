use crate::schema::books;


pub const BOOK_INFO: BookInfo = (
    books::dsl::id,
    books::dsl::title,
    books::dsl::genre,
    books::dsl::user_id
);

pub type BookInfo = (
    books::dsl::id,
    books::dsl::title,
    books::dsl::genre,
    books::dsl::user_id
);