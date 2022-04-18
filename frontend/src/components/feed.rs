use yew::{prelude::*, function_component, html, Html};
use super::TweetBox;

#[function_component(Feed)]
pub fn feed_function() -> Html {
    return html! {
        <>
            <section class="feed"> 
                <div class="feed__header">
                    <h2>{"Home"}</h2>
                </div>

                <TweetBox/>
                // Post 
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