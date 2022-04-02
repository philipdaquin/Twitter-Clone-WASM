use crate::schema::comments;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;


#[derive(Identifiable, Queryable, Deserialize, Serialize)]
#[table_name = "comments"]
pub struct CommentObject { 
    pub id: i32, 
    pub user_id: i32, 
    pub post_id: i32,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Serialize, Insertable, Deserialize)]
#[table_name = "comments"]
pub struct CommentInput { 
    pub user_id: i32, 
    pub body: String
}