pub mod about;
pub mod home;
pub mod follow;
pub mod contact;

use crate::router::{
    contact::Contact,
    follow::Follow,
    home::Home, 
    about::About};
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
    
}

pub fn switch(routes: &AppRoute) -> Html { 
     match routes { 
        AppRoute::Home => html! { <Home/>},
        AppRoute::About => html! { <About/>},
        AppRoute::ContactUs => html! { <Contact/>},
        AppRoute::FollowUser => html! { <Follow/>} 
    }
}