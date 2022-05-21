use std::{env::var, str::FromStr};
use actix_web::{HttpResponse, HttpRequest};
use chrono::{Duration, Local};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, DecodingKey, 
    TokenData, Validation,
    errors::{Error as JsonError, ErrorKind}, 
    EncodingKey, Header};
use strum_macros::{Display, EnumString};

lazy_static! {
    static ref JWT_SECRET_KEY: String = var("JWT_SECRET_KEY").expect("JWT Secret Key Error");
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Claim { 
    issuer: String, 
    subject: String,
    issued_at: i64,
    expiry: i64, 
    login_session: String
}

#[derive(Eq, PartialEq, Display, EnumString, Copy, Clone)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Role { 
    Admin, 
    User
}

pub fn generate_token(username: String, role: Role) -> String { 
    let issuer = var("DOMAIN")
        .unwrap_or_else(|_| "LocalHost".to_string());
    let expiry = (Local::now() + Duration::minutes(60)).timestamp();
    let now = Local::now().timestamp();

    let payload = Claim {
        issuer,
        subject: username,
        issued_at: now,
        expiry,
        login_session: role.to_string(),
    };
    encode(
        &Header::default(),
        &payload, 
        &EncodingKey::from_secret(&JWT_SECRET_KEY.as_ref())
    ).expect("Could not generate JWT Claim")
}

pub fn decode_token(token: &str) -> Result<TokenData<Claim>, JsonError> { 
    Ok(decode::<Claim>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET_KEY.as_ref()),
        &Validation::default(),
    )?)
}

pub fn get_role(req: HttpRequest) -> Option<Role> { 
    req
    .headers()
    .get("Authorization")
    .and_then(|header|  { 
        header.to_str().ok().map(|ch| { 
            let jwt_start_index = "Bearer".len();
            let jwt = ch[jwt_start_index..ch.len()].to_string();
            let token_data = decode_token(&jwt).expect("JsonError");
            Role::from_str(&token_data.claims.login_session).expect("ParseError")
        })
    })
}