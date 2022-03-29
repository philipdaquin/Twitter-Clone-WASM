#[macro_use]
extern crate lazy_static;


use std::env::var;
lazy_static! {
    static ref JWT_SECRET_KEY: String = var("JWT_SECRET_KEY").expect("JWT Secret Key Error");
}

