use std::str::FromStr;
use async_graphql::{Error};
use async_graphql::*;
use serde::{Deserialize, Serialize};
use crate::graphql_module::context::get_redis_conn_from_ctx;
use crate::graphql_module::{
    context::{get_conn_from_ctx},
    modules::utils::{hash_password, verify_password, is_admin},
};
use crate::graphql_module::modules::utils::Role;
use crate::redis::{get_post_cache_key, create_connection};
use chrono::NaiveDateTime;
use super::model::{NewUser, UserObject};
use crate::graphql_module::schema::AppSchema;
use crate::graphql_module::modules;
use common_utils::token::Role as AuthRole;
use crate::graphql_module::modules::utils::RoleGuard;
// use crate::graphql_module::modules::user_model::provider;
use super::provider;
use redis::{aio::ConnectionManager, Value,  AsyncCommands, RedisError};

#[derive(Default)]
pub struct AuthUser;

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct User { 
    pub id: ID, 
    pub created_at: NaiveDateTime,
    pub first_name: String, 
    pub last_name: String, 
    pub username: String, 
    pub location: Option<String>,
    pub email: String, 
    pub hash: String,
    pub role: String
}

#[Object]
impl AuthUser  { 
    /// Reference Resolver for User Details
    #[graphql(entity)]
    pub async fn get_user_details(&self, ctx: &Context<'_>, id: ID) -> Result<Option<User>, Error> { 
        find_user_details(ctx, id)
    }
    
    #[graphql(name = "getAllUsers")]
    pub async fn get_all(
        &self, 
        ctx: &Context<'_>
    ) -> Result<Vec<User>, Error> { 
    
        let conn = &get_conn_from_ctx(ctx);
        let users = provider::get_all_users(conn)
            .expect("Cannot get Users")
            .iter()
            .map(User::from)
            .collect();
        Ok(users)
    }
    
    #[graphql(name = "getAllbyEmail")]
    pub async fn get_users_by_email(
        &self, 
        ctx: &Context<'_>, 
        user_email: String
    ) -> Result<Option<User>, Error> { 
    
        let conn = &get_conn_from_ctx(ctx);
        let user = provider::get_user_by_email(user_email, conn)
            .ok()
            .map(|x| User::from(&x));
        Ok(user)
    }
    
    #[graphql(
        name = "getAllbyId", // guard = "RoleGuard::new(AuthRole::Admin)", // visible = "is_admin"
    )]
    /// This is the test of a description 
    pub async fn get_users_by_id(&self, ctx: &Context<'_>, id: ID, ) -> Option<User> { 
        let cache_key = get_post_cache_key(id.to_string().as_str());
        let redis_client = get_redis_conn_from_ctx(ctx).await;
        let mut redis_connection = create_connection(redis_client)
            .await
            .expect("Unable to create Redis DB connection");
        let cached_object = redis_connection.get(cache_key.clone()).await.expect("Redis Error on Client ");
        
            //  Check If Cache Object is available 
        match cached_object { 
            Value::Nil => { 
                log::info!("Unable to find cache under this id, accessing Database.. 😂");

                let user = find_user_details(ctx, id).expect("Unable to get User Details");
                let _: () = redis::pipe()
                    .atomic()
                    .set(&cache_key, user.clone())
                    .expire(&cache_key, 60)
                    .query_async(&mut redis_connection)
                    .await
                    .expect("Internal Error Occurred while attempting to cache the object");
                return user
            },
            Value::Data(cache) => { 
                log::info!("Cache Found Under this Id! 👌");
                serde_json::from_slice(&cache).expect("Unable to Deserialize Struct")
            },
            _ => { None }
        }
    }
    #[graphql(name = "getAllbyusername", guard = "RoleGuard::new(AuthRole::Admin)", visible = "is_admin")]
    pub async fn get_users_by_username(
        &self, 
        ctx: &Context<'_>,
        user_username: String
    ) -> Result<Option<User>, Error> { 
        let conn = &get_conn_from_ctx(ctx);
        let user = provider::get_user_by_username(user_username, conn)
            .ok()
            .map(|x| User::from(&x));

        Ok(user)
    }
}
pub fn find_user_details(ctx: &Context<'_>, id: ID) -> Result<Option<User>, Error> { 
    let conn = &get_conn_from_ctx(ctx);
    let id = id
        .to_string()   
        .parse::<i32>() 
        .expect("Unable to get Id from String");
    let user = provider::get_user_by_id(id, conn)
        .ok()
        .map(|x| User::from(&x));
    Ok(user)
}

// --------------------------------------
#[derive(Default)]
pub struct UserMutate;
///  User Mutation Classes types
#[derive(InputObject, Deserialize, Serialize, Clone)]
#[graphql(input_name = "userRegisterInput")]
pub struct UserInput { 
    pub username: String,
    pub password: String,
    pub first_name: String, 
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


#[Object]
impl UserMutate { 
    // #[graphql(name = "registerUsers", guard = "RoleGuard::new(AuthRole::Admin)", visible = "is_admin")]
    #[graphql(name = "registerUser")]
    /// This will register the users in our database
    pub async fn register_user(
        &self, 
        ctx: &Context<'_>, 
        user: UserInput
    ) -> Result<User, Error> { 
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
        Ok(User::from(&user_created))
    }
    #[graphql(name = "signInUser")]
    /// The user should be able to 
    pub async fn sign_in(
        &self, 
        ctx: &Context<'_>, 
        input: SignInInput
    ) -> Result<String, Error> { 
        let conn = &get_conn_from_ctx(ctx);
        let get_user = provider::get_user_by_username(input.username, conn);
        if let Some(user) = get_user.ok() { 
            if let Ok(matching) = verify_password(&user.hash,&input.password) { 
                if matching {
                    let role = AuthRole::from_str(user.role.as_str()).expect("Unable to convert to AuthRole");
                    return Ok(common_utils::token::generate_token(user.username, role));
                }
            }
        }
        Err(Error::new("Unable to Authenticate the User"))
    }
}

