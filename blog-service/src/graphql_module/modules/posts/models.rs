use async_graphql::{ID, InputObject};
use diesel::{Queryable, AsChangeset, Insertable};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::schema::posts;
use super::super::users::User;



#[derive(Queryable, Associations, Debug, Serialize, Deserialize, PartialEq, Clone, Identifiable)]
#[table_name = "posts"]
#[belongs_to(User)]
pub struct Post { 
    pub id: i32,
    pub user_id: i32,
    pub slug: String, 
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime, 
    pub title: String, 
    pub description: String, 
    pub body: String,
    pub featured_image: String
}

#[derive(Insertable, Serialize, AsChangeset, Deserialize, Debug, Clone, PartialEq)]
#[table_name = "posts"]
pub struct FormPost { 
    pub slug: Option<String>, 
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>, 
    pub title: Option<String>, 
    pub description: Option<String>, 
    pub body: Option<String>,
    pub featured_image: Option<String>
}
#[derive(InputObject)]
pub struct PostInput { 
    pub slug: String,
    pub title: String, 
    pub description: String, 
    pub body: String,
    pub featured_image: String 
} 
#[derive(Clone, Serialize, Deserialize)]
pub struct PostObject { 
    pub id: ID,
    pub slug: String, 
    pub title: String,
    pub description: String, 
    pub body: String, 
    pub featured_image: String
}


pub const POSTCOLUMNS: PostColumn = (
    posts::id,
    posts::user_id,
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
    posts::user_id,
    posts::slug,
    posts::created_at,
    posts::updated_at,
    posts::title,
    posts::description,
    posts::body,
    posts::featured_image,
);