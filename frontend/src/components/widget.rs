use yew::{prelude::*, function_component, html, Html};
use super::News;
use crate::models::news_widget::NewsWidget;

#[function_component(Widget)]
pub fn widget_function() -> Html {

    //  Insert News Component
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