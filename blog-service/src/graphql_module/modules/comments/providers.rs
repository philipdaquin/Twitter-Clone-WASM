use super::models::{Comment, CommentInput};
use crate::schema::comments;
use crate::schema::posts;
use diesel::prelude::*;
use diesel::dsl::any;


pub fn get_user_comments(id: i32, conn: &PgConnection) -> QueryResult<Vec<Comment>> {
    use crate::schema::comments::dsl::*;
    todo!()
}
pub fn get_comments_by_post(comment_id: i32, conn: &PgConnection) -> QueryResult<Vec<Comment>> { 
    todo!()
}       
pub fn add_comment(input: CommentInput, conn: &PgConnection) -> QueryResult<Comment> {
    todo!()

}
pub fn update_comment(input: CommentInput, conn: &PgConnection) -> QueryResult<Comment> {
    todo!()

}
pub fn remove_comment(id: i32, conn: &PgConnection) -> QueryResult<bool> {
    todo!()

}