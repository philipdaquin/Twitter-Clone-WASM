use yew::{prelude::*, function_component, html, Html};
use crate::models::profileinfo::ProfileObject;
use crate::components::profile::profile_comp::ProfileComp;
// use chrono::{DateTime, Utc, NaiveDateTime};


#[function_component(ProfileFeed)]
pub fn profile_feed() -> Html {
    // let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);

    let user_profile = ProfileObject { 
        id: 1, 
        avatar: "https://pbs.twimg.com/profile_images/736392853992001537/eF4LJLkn_400x400.jpg".to_string(), 
        firstname: "Donald".to_string(), 
        lastname: "Trump".to_string(), 
        username: "".to_string(), 
        bio: Some("Hello akjdklsjd".to_string()), 
        // joined_at: dt
    };
    html! {
        <>
            <section>
                <ProfileComp profile_info={user_profile.clone()}/>
            </section>
        </>
    }
}