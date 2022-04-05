use crate::schema::*;
use async_graphql::{ID, InputObject, Object};
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, NaiveDate};
use crate::schema::posts;
use diesel::prelude::*;
use crate::graphql_module::modules::posts::models::Post;
use crate::schema::comments;

#[derive(Queryable, Identifiable, PartialEq, Clone, Associations, 
    Serialize, Deserialize, Debug)]
#[table_name = "comments"]
#[belongs_to(Post, foreign_key = "post_id")]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostWithComment { 
    pub id: i32, 
    pub title: String, 
    pub published: bool
}

//  Graphql Stuff
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommentObject { 
    pub id: ID, 
    pub user_id: ID, 
    pub post_id: ID, 
    pub body: String, 
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(InputObject)]
pub struct NewComment { 
    pub user_id: ID,
    pub post_id: ID,
    pub body: String,
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

