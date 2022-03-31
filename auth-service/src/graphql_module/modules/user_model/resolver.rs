use async_graphql::*;
use serde::{Deserialize, Serialize};
use crate::graphql_module::{
    context::{get_conn_from_ctx},
    modules::utils::{hash_password, verify_password, is_admin},
};
use super::model::{NewUser, User, UserObject, UserInput};
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
            .expect("")
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

impl UserMutate { 
    // #[graphql(name = "registerUsers", guard = "RoleGuard::new(AuthRole::Admin)", visible = "is_admin")]
    async fn register_user(&self, ctx: &Context<'_>, user: UserInput) -> User { 
        let new_user = NewUser  { 
            first_name: user.first_name,
            last_name: user.last_name,
            username: user.username, 
            
        };
    }
    async fn update_user() -> User {}
    async fn sign_in() -> User { 

    }
    async fn sign_out() -> bool {}
    async fn delete_user() -> bool {}
}
