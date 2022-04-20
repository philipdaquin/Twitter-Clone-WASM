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
                <blockquote class="twitter-tweet">
                <p lang="en" dir="ltr">{"Sunsets don&#39;t get much better than this one over"} 
                 <a href="https://twitter.com/GrandTetonNPS?ref_src=twsrc%5Etfw">{"@GrandTetonNPS"}</a>
                 <a href="https://twitter.com/hashtag/nature?src=hash&amp;ref_src=twsrc%5Etfw">{"#nature"}</a> 
                 <a href="https://twitter.com/hashtag/sunset?src=hash&amp;ref_src=twsrc%5Etfw">{"#sunset"}</a> 
                 <a href="http://t.co/YuKy2rcjyU">{"pic.twitter.com/YuKy2rcjyU"}</a></p>{"&mdash; US Department of the Interior (@Interior) "}
                 <a href="https://twitter.com/Interior/status/463440424141459456?ref_src=twsrc%5Etfw">{"May 5, 2014"}</a></blockquote> 
                 <script async=true src="https://platform.twitter.com/widgets.js" charset="utf-8"></script> 
                
    
                 <a class="twitter-timeline" 
                    href="https://twitter.com/BlaiseKalalo/lists/news-12650?ref_src=twsrc%5Etfw">
                </a> 
                 <script async=true src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

                
                </div>
            </section>
        </>
    }
}