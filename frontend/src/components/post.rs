use yew::{prelude::*, function_component, html, Html};
use chrono::prelude::*;
use crate::models::post::{PostInfo, PostAttributes};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct PostProps { 
    pub post_info: PostInfo,
    pub post_attr: PostAttributes
}


#[function_component(Post)]
pub fn post_info(PostProps { post_info, post_attr }: &PostProps ) -> Html {
    let PostInfo { 
        id, 
        avatar, 
        username, 
        firstname, 
        lastname,
        verified, 
        content,
        media,
        created_at, 
        updated_at} = post_info.clone();

    let PostAttributes { 
        post_id, 
        num_of_likes, 
        num_of_retweets, 
        num_of_comments } = post_attr.clone();

    let verified_user = if verified { 
        " ✅"
    } else { 
        ""
    };

    html! {
        <>  
            <section class="post">
                <div class="post__avatar">
                    <img src={avatar} alt=""/>
                </div>
                <div class="post__body">
                    <div class="post__header">
                        <div class="post__headerText">
                            <h3>
                                {format!("{} {}", firstname, lastname)} 
                                <span class="post__headerSpecial">
                                    {format!("{}", verified_user)}
                                    {format!("{}", username.clone())}
                                    <p>{created_at.format("%B %e, %Y")}</p>
                                </span>
                            </h3>
                        </div>
                        <div class="post__headerDescription"><p>{content}</p></div>
                    </div>
                    <img src={media} alt=""/>
                    <div class="post__footer">
                        <i class="bx bx-message-rounded post_footer_icon">
                            <p class="post_count">{num_of_comments.clone()}</p>
                        </i>
                        <i class="bx bx-repeat post_footer_icon">
                            <p class="post_count">{num_of_retweets.clone()}</p>
                        </i>
                        <i class="bx bx-heart post_footer_icon">
                            <p class="post_count">{num_of_likes.clone()}</p>
                        </i>
                        <i class="bx bx-upload post_footer_icon"></i>
                    </div>
                </div>
            </section>
        </>
    }
}