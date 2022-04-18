use chrono::*;

pub struct PostInfo { 
    pub id: i32, 
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

}
