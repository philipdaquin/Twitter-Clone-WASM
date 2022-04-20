use yew::{prelude::*, function_component, html, Html};
use super::sidebar_options::SidebarOptions;
use yew_router::prelude::*;
use crate::router::AppRoute;


#[function_component(Sidebar)]
pub fn sidebar() -> Html {

    let (home, 
        explore,
        notifications, 
        message, 
        bookmarks,
        lists, 
        profile,
        more,
        search, 
    ) = (
        "bx bx-home-alt",
        "bx bx-hash",
        "bx bx-bell",
        "bx bx-envelope",
        "bx bxs-bookmark",
        "bx bx-detail",
        "bx bx-user",
        "bx bx-dots-horizontal-rounded",
        "bx bx-search"
    );
    
    html! {
        <>
            <section class="sidebar">
                <i class="bx bxl-twitter sidebar__twitterIcon" ></i>
                <Link<AppRoute> to={AppRoute::Home}>
                    <SidebarOptions text={"Home"} icon={home} active={true}/></Link<AppRoute>>
                <SidebarOptions text={"Explore"} icon={search}/>
                <SidebarOptions text={"Notifications"} icon={notifications}/>
                <SidebarOptions text={"Messages"} icon={message}/>
                <SidebarOptions text={"Bookmarks"} icon={bookmarks}/>
                <SidebarOptions text={"Lists"} icon={lists}/>
                <SidebarOptions text={"Profile"} icon={profile}/>
                <SidebarOptions text={"More"} icon={more}/>
                
                <button class="sidebar__tweet" >{"TWEET"}</button>
            </section>
        </>
    }
}