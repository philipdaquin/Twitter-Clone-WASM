use yew::{prelude::*, function_component, html, Html};
use reqwest::{self, Client};
use crate::components::Feed;


#[function_component(Home)]
pub fn home_function() -> Html {
    
    html! {
        <>
            <Feed/>
        </>
    }
}


