use super::model::{Comment, NewComment};
use diesel::prelude::*;
use crate::schema::comments;

pub fn get_all_comments(conn: &PgConnection) -> QueryResult<Vec<Comment>> {
    comments::table.load(conn)
}
pub fn get_comments_by_id(id: i32, conn: &PgConnection) -> QueryResult<Comment> { 
    comments::table.filter(comments::id.eq(id)).first(conn)
}
pub fn get_comments_by_post(post_id: i32, conn: &PgConnection) -> QueryResult<Vec<Comment>> { 
    comments::table.filter(comments::post_id.eq(post_id)).get_results(conn)
}
pub fn get_comments_by_user(user_id: i32, conn: &PgConnection) -> QueryResult<Vec<Comment>> { 
    comments::table.filter(comments::author_id.eq(user_id)).get_results(conn)
}
pub fn get_comment_of_user(post_id: i32, user_id: i32, conn: &PgConnection) -> QueryResult<Vec<Comment>> { 
    comments::table.filter(comments::post_id.eq(post_id))
        .find(user_id)
        .load(conn)
}

pub fn create_comment(new_comment: NewComment, conn: &PgConnection) -> QueryResult<Comment> { 
    diesel::insert_into(comments::table)
        .values(new_comment)
        .get_result::<Comment>(conn)
}

pub fn update_user_comment(post_id: i32, author_id: i32, new_comment: NewComment, conn: &PgConnection) -> QueryResult<Comment> { 
    diesel::update(
        comments::table.filter(comments::author_id.eq(author_id))
        .find(post_id))
        .set(new_comment)
        .get_result::<Comment>(conn)
}

pub fn delete_comment(
    comment_id: i32, 
    post_id: i32, 
    author_id: i32,
    conn: &PgConnection) -> QueryResult<bool> { 
    diesel::delete(comments::table
        //  Select Which Post, Which Comment, and Who's Comment?
        .filter(comments::post_id.eq(post_id))
        .filter(comments::id.eq(comment_id))
        .find(author_id)
    ).execute(conn)?;
    Ok(true)
}