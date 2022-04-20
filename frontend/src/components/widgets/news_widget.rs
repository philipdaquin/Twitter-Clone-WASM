use yew::{prelude::*, function_component, html, Html};
use crate::models::news_widget::{NewsWidget, self};

#[derive(Properties, PartialEq, Clone)]
pub struct WidgetProps { 
    pub news_widget: NewsWidget
}

#[function_component(News)]
pub fn news_function(WidgetProps { news_widget }: &WidgetProps) -> Html {

    let NewsWidget { 
        slug, 
        heading, 
        subheading, 
        media } = news_widget.clone();

    html! {
        <>  
            <section class="widgets">
                <div class="widgets__input">
                    <i class="bx bx-search widgets__searchIcon"></i>
                    <input type="text" placeholder="Search Twitter"/>
                </div>
                <div class="widgets__widgetContainer">
                        
                </div>
            </section>
        </>
    }
}