use super::models::{CommentObject, CommentInput};
use crate::schema::comments;
use diesel::prelude::*;


pub fn get_user_comments(id: i32, conn: &PgConnection) -> QueryResult<CommentObject> {
        
}

pub fn add_comment(input: CommentInput, conn: &PgConnection) -> QueryResult<CommentObject> {

}

pub fn update_comment(input: CommentInput, conn: &PgConnection) -> QueryResult<CommentObject> {

}

pub fn remove_comment(id: i32, conn: &PgConnection) -> QueryResult<bool> {

}