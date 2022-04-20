use yew::{prelude::*, function_component, html, Html};
use crate::models::profileinfo::ProfileObject;
use crate::components::profile::profile_comp::ProfileComp;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct ProfileProps { 
    #[prop_or_default]
    pub user_id: i32,

}
#[function_component(ProfilePage)]
pub fn profile_info(ProfileProps {user_id}: &ProfileProps) -> Html {
    
    //  Query data from database using UserId and Generate this:
    let user_profile = ProfileObject { 
        id: todo!(), 
        avatar: todo!(), 
        firstname: todo!(), 
        lastname: todo!(), 
        username: todo!(), 
        bio: todo!(), 
        joined_at: todo!() 
    };


    html! {
        <>
            <section>
                <ProfileComp profile_info={user_profile.clone()}/>
            </section>
        </>
    }
}