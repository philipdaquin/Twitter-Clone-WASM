use crate::schema::comments;
use async_graphql::ID;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use super::schema::{CommentQuery, User, PostObject, CommentType, CommentInput};
use serde::{Deserialize, Serialize};
#[derive(Queryable, Identifiable, Clone, PartialEq, Debug,
    Deserialize, Serialize)]
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

#[derive(Insertable, AsChangeset, Clone, PartialEq,
    Serialize, Deserialize)]
#[table_name = "comments"]
pub struct NewComment { 
    pub author_id: i32, 
    pub post_id: i32, 
    pub body: String,
    pub media: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}
/// Comment --> CommentType
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

/// Comment_Graphql ---> NewCommment 
impl From<&CommentInput> for NewComment { 
    fn from(f: &CommentInput) -> Self {
        let parse = |id: ID| -> i32 { id.parse::<i32>().expect("")};
        Self { 
            author_id: parse(f.author_id.clone()),
            post_id: parse(f.post_id.clone()),
            body: f.body.clone(),
            media: f.media.clone().unwrap(),
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