use diesel::{Queryable, AsChangeset};
use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::schema::posts;

#[derive(Serialize, Identifiable, Deserialize, Debug, Queryable, Clone)]
#[table_name = "posts"]
pub struct Post { 
    pub id: Uuid,
    pub author_id: Uuid,
    pub slug: String, 
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime, 
    pub title: String, 
    pub description: String, 
    pub body: String,
    pub featured_image: String
}


pub struct FormPost { 
    pub id: Uuid,
    pub author_id: Uuid,
    pub slug: String, 
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime, 
    pub title: String, 
    pub description: String, 
    pub body: String,
    pub featured_image: String
}