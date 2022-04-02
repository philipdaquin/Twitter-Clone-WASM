use diesel::prelude::*;
use super::models::{FormPost, PostObject};
use crate::schema::posts;
use super::models::POSTCOLUMNS;
use diesel::result::Error as DbError;
pub fn get_all(conn: &PgConnection) -> QueryResult<Vec<PostObject>> { 
    use crate::schema::posts::dsl::*;
    posts.load(conn)
}

pub fn get_id(id: i32, conn: &PgConnection) -> QueryResult<PostObject> {
    posts::table
        .select(POSTCOLUMNS)
        .find(id)
        .first::<PostObject>(conn)
}
pub fn get_by_author(author_id: i32, conn: &PgConnection) -> QueryResult<Vec<PostObject>> { 
    use crate::schema::posts::dsl::*;
    posts.find(author_id).load(conn)
}
// pub fn get_for_user(conn: &PgConnection, user_id: i32) -> QueryResult<>
pub fn create_post(form: FormPost, conn: &PgConnection) -> QueryResult<PostObject> {
    //  insert user_id as author_id 
    diesel::insert_into(posts::table)
        .values(form)
        .returning(POSTCOLUMNS)
        .on_conflict_do_nothing()
        .get_result::<PostObject>(conn)
}
pub fn delete_post(post_author: i32, post_id: i32, conn: &PgConnection) -> QueryResult<bool> { 
    use crate::schema::posts::dsl::*;

    diesel::delete(posts
        .filter(author_id.eq(post_author))
        .find(post_id)
    )
    .execute(conn)?;
    
    Ok(true)
}

pub fn update_post(form: FormPost, conn: &PgConnection) -> QueryResult<PostObject> { 
    // let post_id = form.id.ok_or(DbError::NotFound)?;
    // diesel::update(posts::table.find(post_id))
    //     .set(form)
    //     .returning(POSTCOLUMNS)
    //     .get_result::<PostObject>(conn)
    todo!()
}
