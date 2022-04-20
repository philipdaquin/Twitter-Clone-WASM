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
                <h2>{"What's Happenning?"}</h2>
                <div class="widgets__widgetContainer">
                    <h3>{slug}</h3>
                    <h2>{heading}</h2>
                    <p>{subheading}</p>
                    <div>
                        <img src={media} alt=""/>
                    </div>
                </div>
            </section>
        </>
    }
}

