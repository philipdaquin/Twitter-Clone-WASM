use diesel::dsl::*;
use diesel::prelude::*;
use super::models::{BookEntity, NewBook};
use crate::schema::books;
use crate::schema::books::dsl::*;
use super::types::BOOK_INFO;
use crate::db::DbPool;

pub fn get_all(conn: &PgConnection) -> QueryResult<Vec<BookEntity>> {
    books.load::<BookEntity>(conn)        
}
pub fn get_book(id_: i32, conn: &PgConnection) -> QueryResult<BookEntity> { 
    books::table.find(id_).get_result(conn)
}
pub fn get_book_by_user_id(book_id: i32, conn: &PgConnection) -> QueryResult<Vec<BookEntity>> { 
    books::table.filter(books::id.eq(book_id)).load(conn)
}
pub fn create_book(form: NewBook, conn: &PgConnection) -> QueryResult<BookEntity> { 
    let new_book = diesel::insert_into(books)
        .values(form)
        .returning(BOOK_INFO)
        .get_result::<BookEntity>(conn)?;
    Ok(new_book)
}
//  I need user id in the context here 
// pub fn destroy_book(book_id: i32, conn: &PgConnection) -> QueryResult<bool> { 
//     diesel::delete(
//         books.filter(user_id.eq())
//     )
// }