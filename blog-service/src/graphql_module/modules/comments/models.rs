use crate::schema::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::posts;
use diesel::prelude::*;
use crate::graphql_module::modules::posts::models::Post;
use crate::schema::comments;

#[derive(Queryable, Identifiable, PartialEq, Clone, Associations, 
    Serialize, Deserialize, Debug)]
#[table_name = "comments"]
#[belongs_to(Post)]
pub struct Comment { 
    pub id: i32, 
    pub user_id: i32, 
    pub post_id: i32,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Queryable, AsChangeset, Insertable, Deserialize)]
#[table_name = "comments"]
pub struct CommentInput { 
    pub post_id: i32, 
    pub user_id: i32, 
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}

pub const COMMENTOBJECT: CommentObjects = (
    comments::id,
    comments::user_id,
    comments::post_id,
    comments::body,
    comments::created_at,
    comments::updated_at,
);

pub type CommentObjects = (
    comments::id,
    comments::user_id,
    comments::post_id,
    comments::body,
    comments::created_at,
    comments::updated_at,
);

