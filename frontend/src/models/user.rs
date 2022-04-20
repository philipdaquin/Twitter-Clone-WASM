use chrono::{};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserInfo { 
    pub id: i32,
    pub email: String, 
    pub token: String, 
    pub bio: Option<String>,
    pub image: Option<String>,

}


