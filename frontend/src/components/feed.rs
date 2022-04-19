use yew::{prelude::*, function_component, html, Html};
use super::{TweetBox, Post};
use crate::types::post::PostInfo;
use chrono::prelude::*;

#[function_component(Feed)]
pub fn feed_function() -> Html {
    
    let user_post = PostInfo { 
        id: 1,
        avatar: "../../assets/img/host.png ".to_string(),
        username: "@johnapple".to_string(),
        firstname: "Steve".to_string(),
        lastname: "Jobs".to_string(),
        verified: true, 
        content: "This is my first post".to_string(),
        // created_at: chrono::offset::Utc::now(),
        // updated_at: chrono::offset::Utc::now(),
    };

    return html! {
        <>
            <section class="feed"> 
                <div class="feed__header">
                    <h2>{"Home"}</h2>
                </div>

                <TweetBox/>
                
                <Post 
                    post_info={user_post}
                />
                 
                // Post 
                // Post 
                // Post 
                // Post 
                // Post 
                // Post

            </section>
        </>
    }
}