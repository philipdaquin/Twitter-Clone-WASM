use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::{AppRoute, switch};

#[function_component(Navbar)]
pub fn navbar() -> Html { 
    html! {
        <>
        <header class="navbar">
            <div>
                <nav>
                    <ul>
                        <li>
                            <Link<AppRoute> to={AppRoute::Home}><a>{"Home"}</a></Link<AppRoute>>
                        </li>
                        <li>
                            <Link<AppRoute> to={AppRoute::About}><a>{"About"}</a></Link<AppRoute>>
                        </li>
                    </ul>
                </nav>
            </div>
        </header>
        </>
    }
}