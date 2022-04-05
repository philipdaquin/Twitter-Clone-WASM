use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Identifiable, Debug, Clone, PartialEq, 
    Serialize, Deserialize, Queryable)]
#[table_name = "users"]
pub struct UserObject { 
    pub id: i32, 
    pub created_at: NaiveDateTime,
    pub first_name: String, 
    pub last_name: String, 
    pub username: String, 
    pub location: Option<String>,
    pub email: String, 
    pub hash: String,
    pub role: String
}
