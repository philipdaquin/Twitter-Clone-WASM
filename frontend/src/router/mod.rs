pub mod about;
pub mod home;
pub mod follow;
pub mod contact;
pub mod full_post;
pub mod profile;

use crate::router::{
    contact::Contact,
    follow::Follow,
    home::Home, 
    about::About,
    full_post::FullPost,
    profile::Profile
};
use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum AppRoute { 
    #[at("/")]
    Home,

    #[at("/about")]
    About,

    #[at("/contact")]
    ContactUs,

    #[at("/follow")]
    FollowUser,

    #[at("/status/{post_id}")]
    FullPost { post_id: i32 },

    
    #[at("/status/{user_id}")]
    UserProfile { user_id: i32 },
    
}

pub fn switch(routes: &AppRoute) -> Html { 
     match routes { 
        AppRoute::Home => html! { <Home/>},
        AppRoute::About => html! { <About/>},
        AppRoute::ContactUs => html! { <Contact/>},
        AppRoute::FollowUser => html! { <Follow/>},
        AppRoute::FullPost { post_id } => html! { <FullPost post_id={*post_id} />},
        AppRoute::UserProfile { user_id  } => html! { <Profile user_id={*user_id} />}
    }
}