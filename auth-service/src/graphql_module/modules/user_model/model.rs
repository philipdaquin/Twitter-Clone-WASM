use crate::schema::users;
use chrono::{NaiveDateTime, Local};
use common::token::Role;
use diesel::AsChangeset;
use serde::{Serialize, Deserialize};

#[derive(Identifiable, Debug, Clone, PartialEq,
    Serialize, Deserialize, Queryable)]
#[table_name = "users"]
pub struct UserObject { 
    id: i32, 
    created_at: NaiveDateTime,
    first_name: String, 
    last_name: String, 
    username: String, 
    location: Option<String>,
    email: String, 
    hash: String,
    role: String
}

#[derive(Insertable, Deserialize, 
    Serialize, Debug, AsChangeset, Clone, PartialEq)]
#[table_name = "users"]
pub struct NewUser { 
    id: i32, 
    created_at: NaiveDateTime,
    first_name: String, 
    last_name: String, 
    username: String, 
    location: Option<String>,
    email: String, 
    hash: String,
    role: String
} 

use async_graphql::*;
#[derive(SimpleObject)]
pub struct User { 
    username: String, 
    password: String, 
    first_name: String,
    last_name: String, 
    role: Role
}