use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewsWidget { 
    pub slug: String, 
    pub heading: String, 
    pub subheading: String,
    pub media: Option<String>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewsWidgetWrapper { 
    pub new_wrapper: NewsWidget
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewsWidgetList { 
    pub news_widget: Vec<NewsWidget>
}