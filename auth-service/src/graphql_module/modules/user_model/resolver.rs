use async_graphql::*;
use serde::{Deserialize, Serialize};
use common::token::Role as AuthRole;
use crate::graphql_module::{
    context::{get_conn_from_ctx},
    modules::utils::{hash_password, verify_password},
};
use super::model::{NewUser, UserObject};
use crate::graphql_module::schema::AppSchema;
use super::provider::{get_all_users};


#[derive(Default)]
pub struct AuthUser;

#[Object]
impl Authuser  { 
    async fn get_all(&self, ctx: &Context<'_>) -> Vec<UserObject> { 
        get_all_users(get_conn_from_ctx(ctx))
            .expect("")
            .iter()
            .map()
    }
}