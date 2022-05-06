use std::str::FromStr;
use crate::schema::users;
use async_graphql::{SimpleObject, InputObject, ID};
use chrono::{NaiveDateTime, Local};
use diesel::AsChangeset;
use serde::{Serialize, Deserialize};
use super::resolver::User;
use crate::graphql_module::modules::utils::Role;

//  Database Models
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
///  User Query Related Classes
#[derive(Insertable, Deserialize, SimpleObject,
    Serialize, Debug, AsChangeset, Clone, PartialEq)]
#[table_name = "users"]
pub struct NewUser { 
    pub first_name: String, 
    pub last_name: String, 
    pub username: String, 
    pub location: Option<String>,
    pub email: String, 
    pub hash: String,
    pub role: String
}

impl From<&UserObject> for User { 
    fn from(oop: &UserObject) -> Self {
        User { 
            id: oop.id.into(),
            created_at: oop.created_at.clone(),
            location: oop.location.clone(),
            username: oop.username.clone(),
            first_name: oop.first_name.clone(),
            last_name: oop.last_name.clone(),
            email: oop.email.clone(),
            hash: oop.hash.clone(),
            role: Role::from_str(oop.role.as_str())
                .expect("Str to Role Conversion Error")
                .to_string()
        }
    }
}


