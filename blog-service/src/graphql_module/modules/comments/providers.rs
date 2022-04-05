use super::models::{Comment, CommentInput};
use crate::schema::comments;
use crate::schema::posts;
use auth_service::graphql_module::modules::user_model::model::UserObject;
use diesel::prelude::*;
use diesel::dsl::any;
use crate::schema::comments::dsl;
use super::models::COMMENTOBJECT;
use crate::schema::user_comment;

pub fn get_all_comments(conn: &PgConnection) -> QueryResult<Vec<Comment>> {
    comments::table
        .order(comments::id.desc())
        .load::<Comment>(conn)
}
pub fn get_comments_by_post(post_id: i32, conn: &PgConnection) -> QueryResult<Vec<(Comment, UserObject)>> { 
    comments::table
        .filter(comments::post_id.eq(post_id))
        .inner_join(posts::table)
        .select((comments::all_columns, (user_comment::id, user_comment::username)))
        .load::<(Comment, UserObject)>(conn)2
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