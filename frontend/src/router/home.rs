use yew::prelude::*;

pub struct Home;

impl Component for Home { 
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <section class=" max-w-7xl mx-auto flex justify-between items-center bg-yellow-400 border-y border-black py-10  lg:py-0">
                    <div class="px-10 space-y-5">
                        <h1 class="text-6xl max-w-xl font-serif">
                            <span class="underline decoration-black decoration-4">{"Medium"}</span>{" is a place to write, read, and connect "}
                        </h1>
                        <h2>{"
                            It's easy and free to post your thinking on any topic and connect with 
                            with millions of readers.
                            "}
                        </h2>
                    </div>
                    <img class="hidden md:inline-flex h-32 lg:h-full" src="https://accountabilitylab.org/wp-content/uploads/2020/03/Medium-logo.png" alt="" />

                    //  Posts 
                </section>
            </>
        }
    }
}