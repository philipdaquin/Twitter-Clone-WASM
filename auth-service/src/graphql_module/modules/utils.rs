use actix_web::Result;
use argonautica::{Error as HasherError, Hasher, Verifier};
use serde::{Serialize, Deserialize};
use strum_macros::{Display, EnumString};
use std::env::var;
use common_utils::token::Role as AuthRole;
use jsonwebtoken;
use async_graphql::Guard;
use async_graphql::*;


#[derive(Copy, Clone, Eq, PartialEq, Serialize, 
    Deserialize, Enum, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    Admin,
    User,
}

pub struct RoleGuard {
    pub role: AuthRole,
}

impl RoleGuard {
    pub fn new(role: AuthRole) -> Self {
        Self { role }
    }
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, context: &Context<'_>) -> Result<(), async_graphql::Error> {
        
        if context.data_opt::<AuthRole>() == Some(&AuthRole::Admin) || context.data_opt::<AuthRole>() == Some(&self.role) {
            Ok(())
        } else {
            let guard_error = context.data_opt::<jsonwebtoken::errors::Error>().clone();
            match guard_error {
                Some(e) => return Err(format!("{:?}", e.kind()).into()),
                None => return Err(format!("Access denied: {} role required", &self.role).into())
            }
        }
    }
}

/// Field will only be visible to users with Role::Admin
pub fn is_admin(ctx: &Context<'_>) -> bool {
    ctx.data_opt::<Role>() == Some(&Role::Admin)
}


lazy_static! { 
    static ref PASSWORD_SECRET_KEY: String = 
        var("PASSWORD_SECRET_KEY")
        .expect("You must set PASSWORD_SECRET_KEY");
}

pub fn hash_password(pwd: &str) -> Result<String, HasherError> { 
    Hasher::default()
        .with_password(pwd)
        .with_secret_key(PASSWORD_SECRET_KEY.as_str())
        .hash()
} 
pub fn verify_password(hash: &str, pwd: &str) -> Result<bool, HasherError> { 
    Verifier::default()
        .with_hash(hash)
        .with_password(pwd)
        .with_secret_key(PASSWORD_SECRET_KEY.as_str())
        .verify()
}

