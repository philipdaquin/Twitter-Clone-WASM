use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Identifiable, Debug, Clone, PartialEq, Queryable,
    Serialize, Deserialize)]
#[table_name = "users"]
pub struct User { 
    pub id: i32, 
    pub created_at: NaiveDateTime,
    pub first_name: String, 
    pub last_name: String, 
    pub username: String, 
    pub location: Option<String>,
    pub email: String, 
}
