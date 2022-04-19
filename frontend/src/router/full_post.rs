use yew::{prelude::*, function_component, html, Html};

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct FullPostProps { 
    #[prop_or_default]
    pub post_id: i32
}
#[function_component(FullPost)]
pub fn fullpost(FullPostProps {post_id}: &FullPostProps) -> Html {
    html! {
        <>
            <section>
                {"FullPost"}
            </section>
        </>
    }
}