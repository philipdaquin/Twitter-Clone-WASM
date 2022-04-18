use yew::{prelude::*, function_component, html, Html};
use chrono::prelude::*;


pub struct PostProps { 
    id: i32, 
    username: String, 
    firstname: String, 
    lastname: String, 
    verified: bool, 
    created_at: DateTime<Utc>
}


#[function_component(Post)]
pub fn post_info() -> Html {
    html! {
        <>  
            <section class="post"></section>
        </>
    }
}