use diesel::prelude::*;
use super::models::{FormPost, Post};
use crate::schema::posts;

pub fn get(conn: &PgConnection) -> QueryResult<Vec<Post>> { 
    use crate::schema::posts::dsl::*;
    posts.load(conn)
}
