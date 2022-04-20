use chrono::DateTime;

use chrono::prelude::*;
use serde::Deserialize;
use serde::Serialize;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileObject { 
    pub id: i32, 
    pub avatar: String, 
    pub firstname: String, 
    pub lastname: String, 
    pub username: String,
    pub bio: Option<String>, 
    // pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileObjectWrapper { 
    pub user_profile: ProfileObject
}
