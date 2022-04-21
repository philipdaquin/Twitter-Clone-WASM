use yew::{prelude::*, function_component, html, Html};
use crate::models::{
    profileinfo::ProfileObject,
};
use crate::components::profile_feed::ProfileFeed;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct ProfileProps { 
    #[prop_or_default]
    pub username: String,

}
#[function_component(ProfilePage)]
pub fn profile_info(ProfileProps {username}: &ProfileProps) -> Html {
    
    


    html! {
        <>
            // <img src="https://media.vanityfair.com/photos/6224ed983b4f9b2db0035fff/master/pass/Donald-Trump.jpg" alt=""/>
            <section>
                <ProfileFeed/>                
            </section>
        </>
    }
}