use std::str::FromStr;

use async_graphql::*;
use serde::{Deserialize, Serialize};
use crate::graphql_module::{
    context::{get_conn_from_ctx},
    modules::utils::{hash_password, verify_password, is_admin},
};
use super::model::{NewUser, User, UserObject, UserInput, SignInInput};
use crate::graphql_module::schema::AppSchema;
use crate::graphql_module::modules;
use common::token::Role as AuthRole;
use crate::graphql_module::modules::utils::RoleGuard;
// use crate::graphql_module::modules::user_model::provider;
use super::provider;


#[derive(Default)]
pub struct AuthUser;

#[Object]
impl AuthUser  { 
    #[graphql(name = "getAllUsers", guard = "RoleGuard::new(AuthRole::Admin)", visible = "is_admin")]
    pub async fn get_all(&self, ctx: &Context<'_>) -> Vec<User> { 
        let conn = &get_conn_from_ctx(ctx);
        provider::get_all_users(conn)
            .expect("Cannot get Users")
            .iter()
            .map(User::from)
            .collect()
    }
    #[graphql(name = "getAllbyEmail", guard = "RoleGuard::new(AuthRole::Admin)", visible = "is_admin")]
    pub async fn get_users_by_email(&self, ctx: &Context<'_>, user_email: String) -> Option<User> { 
        let conn = &get_conn_from_ctx(ctx);
        provider::get_user_by_email(user_email, conn)
            .ok()
            .map(|x| User::from(&x))
    }
    #[graphql(name = "getAllbyId", guard = "RoleGuard::new(AuthRole::Admin)", visible = "is_admin")]
    pub async fn get_users_by_id(&self, ctx: &Context<'_>, id: ID) -> Option<User> { 
        let conn = &get_conn_from_ctx(ctx);
        let id = id
            .to_string()   
            .parse::<i32>() 
            .expect("Unable to get Id from String");
        provider::get_user_by_id(id, conn)
            .ok()
            .map(|x| User::from(&x))
    }
    #[graphql(name = "getAllbyusername", guard = "RoleGuard::new(AuthRole::Admin)", visible = "is_admin")]
    pub async fn get_users_by_username(&self, ctx: &Context<'_>, user_username: String) -> Option<User> { 
        let conn = &get_conn_from_ctx(ctx);
        provider::get_user_by_username(user_username, conn)
            .ok()
            .map(|x| User::from(&x))
    }
}

#[derive(Default)]
pub struct UserMutate;

#[Object]
impl UserMutate { 
    #[graphql(name = "registerUsers", guard = "RoleGuard::new(AuthRole::Admin)", visible = "is_admin")]
    pub async fn register_user(&self, ctx: &Context<'_>, user: UserInput) -> User { 
        let conn = &get_conn_from_ctx(ctx);
        
        let new_user = NewUser  { 
            first_name: user.first_name,
            last_name: user.last_name,
            username: user.username, 
            location: Some(user.location),
            email: user.email,
            hash: hash_password(user.password.as_str()).expect("Unable to hash user password"),
            role: user.role.to_string()
        };
        let user_created = provider::create_user(new_user, conn).expect("Cannot create user right now");
        User::from(&user_created)
    }
    pub async fn sign_in(&self, ctx: &Context<'_>, input: SignInInput) -> Result<String, Error> { 
        let conn = &get_conn_from_ctx(ctx);
        
        let get_user = provider::get_user_by_username(input.username, conn);
        if let Some(user) = get_user.ok() { 
            if let Ok(matching) = verify_password(&user.hash,&input.password) { 
                if matching {
                    let role = AuthRole::from_str(user.role.as_str()).expect("Unable to convert to AuthRole");
                    return Ok(common::token::generate_token(user.username, role));
                }
            }
        }
        Err(Error::new("Unable to Authenticate the User"))
    }
    // async fn sign_out() -> bool {}
    // async fn delete_user() -> bool {}
}
