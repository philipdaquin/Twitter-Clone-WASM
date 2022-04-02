use super::models::{CommentObject, CommentInput};
use crate::schema::comments;
use diesel::prelude::*;
use diesel::dsl::any;

pub fn get_user_comments(id: i32, conn: &PgConnection) -> QueryResult<Vec<CommentObject>> {
    todo!()
}
pub fn add_comment(input: CommentInput, conn: &PgConnection) -> QueryResult<CommentObject> {
    todo!()

}
pub fn update_comment(input: CommentInput, conn: &PgConnection) -> QueryResult<CommentObject> {
    todo!()

}
pub fn remove_comment(id: i32, conn: &PgConnection) -> QueryResult<bool> {
    todo!()

}