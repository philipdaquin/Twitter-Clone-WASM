use yew::{prelude::*, function_component, html, Html};
use crate::models::profileinfo::ProfileObject;

#[derive(Properties, PartialEq, Clone)]
pub struct ProfileObjectProps { 
    pub profile_info: ProfileObject
}


#[function_component(ProfileComp)]
pub fn profile_function(ProfileObjectProps { profile_info}: &ProfileObjectProps) -> Html {
    
    let ProfileObject { 
        id, 
        avatar, 
        firstname, 
        lastname, 
        username, 
        bio,
        // joined_at
     } = profile_info.clone();
    
    return html! {
        <>
            <section class="">
                <div><img src={avatar} alt=""/></div>
                <h2>{format!("{} {}", firstname, lastname)}</h2>
                <p>{username}</p>
                <p>{bio.unwrap_or_default()}</p>
                <p>{
                    // joined_at.format("%B %e, %Y") ""
                    "Hello"
                }</p>
            </section>
        </>
    }
}