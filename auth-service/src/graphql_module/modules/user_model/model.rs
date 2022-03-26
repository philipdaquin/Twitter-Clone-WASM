use crate::schema::users;
use chrono::{NaiveDateTime, Local};
use serde::{Serialize, Deserialize};

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable)]
#[table_name = "users"]
pub struct UserObject { 
    id: i32, 
    created_at: NaiveDateTime,
    
    first_name: String, 
    last_name: String, 
    username: String, 
    location: String,
    email: String, 
    hash: String,
    
    role: String
}

#[derive(Insertable, Deserialize, Serialize, Debug)]
#[table_name = "users"]
pub struct NewUser { 
    id: i32, 
    created_at: NaiveDateTime,
    
    first_name: String, 
    last_name: String, 
    username: String, 
    location: String,
    email: String, 
    hash: String,
    
    role: String
} 