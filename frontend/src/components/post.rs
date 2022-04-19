use yew::{prelude::*, function_component, html, Html};
use chrono::prelude::*;
use crate::types::post::PostInfo;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct PostProps { 
    pub post_info: PostInfo
}


#[function_component(Post)]
pub fn post_info(PostProps { post_info }: &PostProps ) -> Html {
    let PostInfo { 
        id, 
        avatar, 
        username, 
        firstname, 
        lastname,
        verified, 
        content,
        created_at, 
        updated_at
    } = post_info.clone();

    let verified_user = if verified { 
        html! {"✅"}
    } else { 
        html! {}
    };
    

    html! {
        <>  
            <section class="post">
                <div class="post__avatar">
                    <img src={avatar} alt=""/>
                </div>
                <div class="post__body">
                    <div class="post__">
                        <div class="post__headerText">
                            <h3>
                                {format!("{:?} {:?}", firstname, lastname)} <span>
                                    {format!("{:?}", verified_user)}
                                </span>
                                <span>
                                    {created_at.format("%B %e, %Y")}
                                </span>
                            </h3>
                        </div>
                        <div class="post__headerDescription">
                            <p>{content}</p>
                        </div>
                    </div>
                    <img src="" alt=""/>
                    <div class="post__footer">
                        <span class="material-icons-outlined"></span>
                        <i class="bx bx-heart"></i>
                    </div>
                </div>
            </section>
        </>
    }
}