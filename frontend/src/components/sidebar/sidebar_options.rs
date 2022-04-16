use yew::{prelude::*, function_component, html, Html};

#[derive(Properties, Clone, PartialEq)]
pub struct SidebarProps {
    #[prop_or_default] 
    pub text: String,
    #[prop_or_default]
    pub icon: String,
    #[prop_or_default]
    pub active: bool,
}

#[function_component(SidebarOptions)]
pub fn sidebar_options(SidebarProps {
    text, icon, active 
}: &SidebarProps) -> Html {

    let active_prop = if !*active { 
        "sidebarOption"
    } else { 
        "sidebarOption sidebarOption--active"
    };

    html! {
        <>
            <section class={active_prop}>
                <i class={icon}></i>
                <h2>{text}</h2>
            </section>
        </>
    }
}