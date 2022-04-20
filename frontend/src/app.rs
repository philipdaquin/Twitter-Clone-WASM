use yew::prelude::*;
use yew_router::prelude::*;
use crate::{
    components::{
        Header,
        Footer,
        Sidebar,
        Feed,
        Widget
    },
    router::{AppRoute, 
        home::Home,
        about::About,
        switch,
    }
};

pub struct App;

impl Component for App { 
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
                <section class="app">
                    <BrowserRouter>
                        <Sidebar/>
                            <Switch<AppRoute> render={Switch::render(switch)} />
                        <Widget/>
                    </BrowserRouter>
                </section>
            </>
        }
    }
}