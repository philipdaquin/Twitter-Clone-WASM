use async_graphql::{ID, InputObject};
use diesel::{Queryable, AsChangeset, Insertable};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::schema::posts;
use super::resolver::{PostObject, User};


#[derive(Queryable, Debug, Serialize, Deserialize, PartialEq, Clone, Identifiable)]
#[table_name = "posts"]
pub struct Post { 
    pub id: i32,
    pub user_id: i32,
    pub slug: String, 
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime, 
    pub title: String, 
    pub description: String, 
    pub body: String,
    pub featured_image: String
}

#[derive(Insertable, Serialize, AsChangeset, Deserialize, Debug, Clone, PartialEq)]
#[table_name = "posts"]
pub struct FormPost { 
    pub slug: Option<String>, 
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>, 
    pub title: Option<String>, 
    pub description: Option<String>, 
    pub body: Option<String>,
    pub featured_image: Option<String>
}


impl From<&Post> for PostObject { 
    fn from(oop: &Post) -> Self {
        PostObject { 
            id: oop.id.into(),
            user_id: User::convert(oop.id),
            slug: oop.slug.clone(),
            created_at: oop.created_at.clone(),
            updated_at: oop.updated_at.clone(),
            title: oop.title.clone(),
            description: oop.description.clone(),
            body: oop.body.clone(),
            featured_image: oop.featured_image.clone()
        }
    }
}


impl User { 
    fn convert(id: i32) -> User { 
        Self { 
            id: id.into()
        }
    }
}