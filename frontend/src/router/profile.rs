use yew::{prelude::*, function_component, html, Html};
use crate::models::profileinfo::UserProfile;


#[derive(Properties, PartialEq, Clone, Debug)]
pub struct ProfileProps { 
    #[prop_or_default]
    pub user_id: i32
}
#[function_component(Profile)]
pub fn profile_info(ProfileProps {user_id}: &ProfileProps) -> Html {
    // let UserProfile { 
    //     avatar,
    //     firstname, 
    //     lastname, 
    //     username, 
    //     bio,
    //     joined_at, 
    // } = user_profile.clone();


    html! {
        <>
            <section>
                {"Profile"}
            </section>
        </>
    }
}