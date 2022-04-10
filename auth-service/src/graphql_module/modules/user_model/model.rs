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
#[derive(SimpleObject)]
pub struct User { 
    pub username: String, 
    pub first_name: String,
    pub last_name: String, 
    pub role: Role
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
#[derive(InputObject, Deserialize, Serialize, Clone)]
#[graphql(input_name = "userRegisterInput")]
pub struct UserInput { 
    /// Hello
    pub username: String,
    /// Hello
    pub password: String,
    /// Hello 
    pub first_name: String, 
    ///  asdasdasd`
    pub last_name: String, 
    pub location: String,
    pub email: String, 
    pub role: Role
}

#[derive(InputObject, Deserialize, Serialize, Clone)]
#[graphql(input_name = "signInInput")]
pub struct SignInInput { 
    pub username: String, 
    pub password: String 
}












