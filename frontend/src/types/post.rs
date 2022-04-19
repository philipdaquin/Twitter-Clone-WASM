use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostInfo { 
    pub id: i32, 
    pub avatar: String, 
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub verified: bool,
    pub content: String,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostInfoWrapper { 
    pub posts: PostInfo,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostInfoList { 
    pub posts: Vec<PostInfo>
}

