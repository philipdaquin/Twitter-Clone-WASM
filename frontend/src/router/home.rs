use yew::{prelude::*, function_component, html, Html};
use reqwest::{self, Client};
use crate::service::posts::Posts;

#[function_component(Home)]
pub fn home_function() -> Html {
    
    html! {
        <>
            <section class=" max-w-7xl mx-auto flex justify-between items-center bg-yellow-400 border-y border-black py-10  lg:py-0">
                <div class="px-10 space-y-5">
                    <h1 class="text-6xl max-w-xl font-serif">
                        <span class="underline decoration-black decoration-4">{"Medium"} </span>
                        {" is a place to write, read, and connect "}
                    </h1>
                    <h2> {" It's easy and free to post your thinking on any topic and connect with 
                        with millions of readers."} </h2>
                </div>
                <img class="hidden md:inline-flex h-32 lg:h-full" 
                    src="https://accountabilitylab.org/wp-content/uploads/2020/03/Medium-logo.png" 
                    alt="" />
                <div>
                    <Posts/>
                </div>
            </section>
        </>
    }
}


