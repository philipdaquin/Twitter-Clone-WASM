use crate::schema::comments;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use super::schema::{CommentQuery, User, PostObject, CommentType};
#[derive(Queryable, Identifiable, Clone, PartialEq, Debug)]
#[table_name = "comments"]
pub struct Comment { 
    pub id: i32,
    pub author_id: i32, 
    pub post_id: i32,
    pub body: String,
    pub media: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>     
}

#[derive(Insertable, AsChangeset, Clone, PartialEq)]
#[table_name = "comments"]
pub struct NewComment { 
    pub author_id: i32, 
    pub post_id: i32, 
    pub body: String,
    pub media: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}

impl From<&Comment> for CommentType { 
    fn from(f: &Comment) -> Self {
        Self { 
            id: f.id.into(),
            author_id: User::user_id(f.author_id),
            post_id: PostObject::post_id(f.post_id),
            body: f.body.clone(),
            media: f.media.clone(),
            created_at: f.created_at.clone(),
            updated_at: f.updated_at.clone() 
        }
    }
}

impl User { 
    fn user_id(id: i32) -> Self { 
        Self { 
            id: id.into() 
        }
    }
}

impl PostObject { 
    fn post_id(id: i32) -> Self { 
        Self { 
            id: id.into()
        }
    }
}