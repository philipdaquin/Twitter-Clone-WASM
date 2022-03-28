use actix_web::Result;
use argonautica::{Error as HasherError, Hasher, Verifier};
use std::env::var;

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