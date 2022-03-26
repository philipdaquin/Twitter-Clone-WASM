use yew::prelude::*;

pub struct Home;

impl Component for Home { 
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="home">
                    <img class="profile-picture" src="assets/images/avatar.jpg" alt="ShironCat's avatar" />
                    <h1>{ "Hello, World!" }</h1>
                    <img src="https://http.cat/404.jpg" />
                </div>
            </>
        }
    }
}