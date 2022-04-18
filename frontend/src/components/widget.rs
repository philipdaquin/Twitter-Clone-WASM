use yew::{prelude::*, function_component, html, Html};

#[function_component(Widget)]
pub fn widget_function() -> Html {
    html! {
        <>  
            <section class="widgets">
                <h2>{"Widgets "}</h2>
            </section>
        </>
    }
}