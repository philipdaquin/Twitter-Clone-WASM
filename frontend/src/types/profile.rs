use yew::{prelude::*, function_component, html, Html};

#[function_component(Profile)]
pub fn profile_info() -> Html {
    html! {
        <>
            <section>
                {"Profile"}
            </section>
        </>
    }
}