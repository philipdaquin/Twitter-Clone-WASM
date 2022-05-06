use diesel::prelude::*;
use super::models::{FormPost, Post};
use crate::schema::posts;

pub fn get_all(conn: &PgConnection) -> QueryResult<Vec<Post>> { 
    posts::table.load(conn)
}
pub fn get_post_by_id(id: i32, conn: &PgConnection) -> QueryResult<Post> {
    posts::table.filter(posts::id.eq(id)).first::<Post>(conn)
}
pub fn get_by_posts_by_author(author_id: i32, conn: &PgConnection) -> QueryResult<Vec<Post>> { 
    posts::table.filter(posts::user_id.eq(author_id)).load(conn)        
}
// pub fn get_for_user(conn: &PgConnection, user_id: i32) -> QueryResult<>
pub fn create_post(form: FormPost, conn: &PgConnection) -> QueryResult<Post> {
    diesel::insert_into(posts::table)
        .values(form)
        .get_result::<Post>(conn)?;
    posts::table    
        .order(posts::id.desc())
        .select(posts::all_columns)
        .first(conn)
        .map_err(Into::into)
}
pub fn delete_post(post_author: i32, post_id: i32, conn: &PgConnection) -> QueryResult<bool> { 

    diesel::delete(posts::table
        .filter(posts:: user_id.eq(post_author))
        .find(post_id)
    )
    .execute(conn)?;
    
    Ok(true)
}

pub fn update_post(post_id: i32, user_id: i32, form: FormPost, conn: &PgConnection) -> QueryResult<Post> { 
    diesel::update(posts::table
        .filter(posts::user_id.eq(user_id))
        .find(post_id)
    )
    .set(form)
    .get_result::<Post>(conn)
}
