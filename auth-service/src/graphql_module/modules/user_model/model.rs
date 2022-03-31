use std::str::FromStr;
use crate::schema::users;
use async_graphql::{SimpleObject, InputObject, ID};
use chrono::{NaiveDateTime, Local};
use diesel::AsChangeset;
use serde::{Serialize, Deserialize};
use crate::graphql_module::modules::utils::Role;

//  Database Models
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

///  User Query Related Classes
#[derive(Insertable, Deserialize, 
    Serialize, Debug, AsChangeset, Clone, PartialEq)]
#[table_name = "users"]
pub struct NewUser { 
    first_name: String, 
    last_name: String, 
    username: String, 
    location: Option<String>,
    email: String, 
    hash: String,
    role: String
}
#[derive(SimpleObject)]
pub struct User { 
    username: String, 
    first_name: String,
    last_name: String, 
    role: Role
}
impl From<&UserObject> for User { 
    fn from(oop: &UserObject) -> Self {
        User { 
            username: oop.username.clone(),
            first_name: oop.first_name.clone(),
            last_name: oop.last_name.clone(),
            role: Role::from_str(oop.role.as_str()).expect("Str to Role Conversion Error")
        }
    }
}
///  User Mutation Classes types
#[derive(InputObject)]
pub struct UserInput { 
    pub username: String,
    pub password: String, 
    pub first_name: String, 
    pub last_name: String, 
    pub role: Role
}

#[derive(InputObject)]
pub struct SignInInput { 
    username: String, 
    password: String 
}

















