#![recursion_limit = "512"]

pub mod app;
pub mod api;
pub mod service;
pub mod components;
pub mod hooks;
pub mod router;
pub mod models;

extern crate serde;
use crate::app::App;


use wasm_bindgen::prelude::*;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn main() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    Ok(())
}