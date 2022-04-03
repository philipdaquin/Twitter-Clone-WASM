use super::models::{Comment, CommentInput};
use crate::schema::comments;
use crate::schema::posts;
use diesel::prelude::*;
use diesel::dsl::any;
use crate::schema::comments::dsl;
use super::models::COMMENTOBJECT;

pub fn get_all_comments(conn: &PgConnection) -> QueryResult<Vec<Comment>> {
    comments::table
        .order(comments::id.desc())
        .load::<Comment>(conn)
}
pub fn get_comments_by_post(comment_id: i32, conn: &PgConnection) -> QueryResult<Vec<Comment>> { 
    comments::table 
        .filter(comments::post_id.eq(comment_id))
        .load::<Comment>(conn)
}       
pub fn add_comment(input: CommentInput, conn: &PgConnection) -> QueryResult<Comment> {
    diesel::insert_into(comments::table)
        .values(input)        
        .returning(COMMENTOBJECT)
        .on_conflict_do_nothing()
        .get_result::<Comment>(conn)
}
pub fn update_comment(input: CommentInput, conn: &PgConnection) -> QueryResult<Comment> {
    use crate::schema::comments::{post_id};
    diesel::update(comments::table.filter(post_id.eq(input.post_id)))
        .set(input)
        .returning(COMMENTOBJECT)
        .get_result::<Comment>(conn)
}
pub fn remove_comment(id: i32, conn: &PgConnection) -> QueryResult<bool> {
    diesel::delete(comments::table)
        .filter(comments::id.eq(id))
        .execute(conn)?;
    Ok(true)
}