use async_graphql::ID;
use diesel::{Queryable, AsChangeset, Insertable};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::schema::posts;

#[derive(Serialize, Identifiable, Deserialize, AsChangeset, Debug, Queryable, Clone)]
#[table_name = "posts"]
pub struct PostObject { 
    pub id: i32,
    pub author_id: i32,
    pub slug: String, 
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime, 
    pub title: String, 
    pub description: String, 
    pub body: String,
    pub featured_image: String
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[table_name = "posts"]
pub struct FormPost { 
    pub slug: String, 
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>, 
    pub title: String, 
    pub description: String, 
    pub body: String,
    pub featured_image: String
}



pub const POSTCOLUMNS: PostColumn = (
    posts::id,
    posts::author_id,
    posts::slug,
    posts::created_at,
    posts::updated_at,
    posts::title,
    posts::description,
    posts::body,
    posts::featured_image,
);

pub type PostColumn = (
    posts::id,
    posts::author_id,
    posts::slug,
    posts::created_at,
    posts::updated_at,
    posts::title,
    posts::description,
    posts::body,
    posts::featured_image,
);