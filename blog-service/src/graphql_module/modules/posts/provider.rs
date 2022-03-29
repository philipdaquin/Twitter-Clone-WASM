use diesel::prelude::*;
use super::models::{FormPost, Post};
use crate::schema::posts;

pub fn get_all(conn: &PgConnection) -> QueryResult<Vec<Post>> { 
    use crate::schema::posts::dsl::*;

    posts.load(conn)
}
pub fn get_id(conn: &PgConnection) -> QueryResult<Post> {}
pub fn get_for_user(conn: &PgConnection, user_id: i32) -> QueryResult<>

pub fn create() -> QueryResult<> {}


