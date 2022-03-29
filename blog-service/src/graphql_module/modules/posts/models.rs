use diesel::{Queryable, AsChangeset, Insertable};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::schema::posts;

#[derive(Serialize, Identifiable, Deserialize, AsChangeset, Debug, Queryable, Clone)]
#[table_name = "posts"]
pub struct Post { 
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
    pub updated_at: NaiveDateTime, 
    pub title: String, 
    pub description: String, 
    pub body: String,
    pub featured_image: String
}