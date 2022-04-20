use yew::{prelude::*, function_component, html, Html};
use super::{TweetBox, Post};
use crate::models::post::{PostInfo, PostAttributes};
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

#[function_component(Feed)]
pub fn feed_function() -> Html {

    //  Example Data 
    let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);
    let user_post = PostInfo { 
        id: 1,
        avatar: "../../assets/img/host.png ".to_string(),
        username: "@johnapple".to_string(),
        firstname: "Steve".to_string(),
        lastname: "Jobs".to_string(),
        verified: true, 
        content: "This is my first post".to_string(),
        media: "https://cdn.cnn.com/cnnnext/dam/assets/210602130929-donald-trump-file-january-20-2021-super-tease.jpg".to_string(),
        created_at: dt,
        updated_at: dt,
    };

    let attr = PostAttributes {
        post_id: 1, 
        num_of_likes: 222, 
        num_of_retweets: 23, 
        num_of_comments: 12 
    };

    return html! {
        <>
            <section class="feed"> 
                <div class="feed__header">
                    <h2>{"Home"}</h2>
                </div>

                <TweetBox/>
                
                <Post post_info={user_post.clone()} post_attr={attr.clone()}/>
                <Post post_info={user_post.clone()} post_attr={attr.clone()}/>
                <Post post_info={user_post.clone()} post_attr={attr.clone()}/>
                <Post post_info={user_post.clone()} post_attr={attr.clone()}/>
                <Post post_info={user_post.clone()} post_attr={attr.clone()}/>
                 
            </section>
        </>
    }
}