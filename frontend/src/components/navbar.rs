use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::{AppRoute, switch};

#[function_component(Navbar)]
pub fn navbar() -> Html { 
    html! {
        <>
            <header class="flex justify-between p-5 max-w-7xl mx-auto ">
                <div class="flex items-center space-x-5">
                    <Link<AppRoute> to={AppRoute::Home}>
                        <img class="w-44 object-contain cursor-pointer" 
                            src="https://links.papareact.com/yvf" /></Link<AppRoute>>
                    <div class="hidden md:inline-flex items-center space-x-5">
                        <h3> <Link<AppRoute> to={AppRoute::About}><a>{"About"}</a></Link<AppRoute>></h3>
                        <h3> <Link<AppRoute> to={AppRoute::ContactUs}><a>{"Contact"}</a></Link<AppRoute>></h3>
                        <h3 class="text-white bg-green-600 px-4 py-1 rounded-full"> 
                            <Link<AppRoute> to={AppRoute::FollowUser}><a>{"Follow"}</a></Link<AppRoute>></h3>
                    </div>
                </div>

                <div class="flex items-center space-x-5 text-green-600">
                    <h3>{"Sign In"}</h3>
                    <h3 class="border px-4 py-1 rounded-full border-green-600">{"Get Started"}</h3>
                </div>
            </header>
        </>
    }
}