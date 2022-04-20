use yew::{prelude::*, function_component, html, Html};
use crate::models::user::UserInfo;


#[derive(Properties, Clone, Debug, PartialEq)]
pub struct TweetProp { 
    pub user: UserInfo
}

#[function_component(TweetBox)]
pub fn tweetbox() -> Html {
    
    // let UserInfo { 
    //     image, ..
    // } = user.clone();

    html! {
        <>  
            <div class="tweetbox">
                <form action="">
                    <div class="tweetBox__input">
                        <img src={""} class="" />
                        <input type="text" placeholder="What's happening today"/>
                    </div>
                    <input type="text" placeholder="Optional: Enter image URL" class="tweetBox__imageInput"/>

                    <button class="tweetBox__tweetButton">{"Tweet"}</button>
                </form>
            </div>
        </>
    }
}