use super::models::{Comment, CommentInput};
use crate::schema::comments;
use crate::schema::posts;
use diesel::prelude::*;
use diesel::dsl::any;
use crate::schema::comments::dsl;
use super::models::{COMMENTOBJECT, CommentUser, PostUser};
use crate::schema::users;
use super::super::users::User;




pub fn get_all_comments(conn: &PgConnection) -> QueryResult<Vec<Comment>> {
    comments::table
        .order(comments::id.desc())
        .load::<Comment>(conn)
}
pub fn get_comments_by_post(post_id: i32, conn: &PgConnection) -> QueryResult<Vec<CommentUser>> { 
    comments::table 
        .filter(comments::post_id.eq(post_id))
        .inner_join(users::table)
        .select((comments::all_columns, users::all_columns))
        .load::<(Comment, User)>(conn)
        .map_err(Into::into)
}       
pub fn add_comment(user_id: i32, post_id: i32, body: &str, conn: &PgConnection) -> QueryResult<Comment> {
    diesel::insert_into(comments::table)
        .values(
        (
            comments::user_id.eq(user_id),
            comments::post_id.eq(post_id),
            comments::body.eq(body)
            )
        )
        .execute(conn)?;
        comments::table 
            .order(comments::id.desc())
            .select(comments::all_columns)
            .first(conn)
            .map_err(Into::into)    
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