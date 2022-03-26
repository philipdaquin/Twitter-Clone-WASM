use diesel::Insertable;
use serde::{Serialize, Deserialize};

use crate::schema::books;

#[derive(Identifiable, Serialize, Deserialize, 
    Queryable, Debug, Clone, PartialEq)]
#[table_name = "books"]
pub struct BookEntity { 
    pub id: i32, 
    pub title: String,
    pub genre: String, 
    pub user_id: i32
}

#[derive(Insertable, Deserialize, Serialize, 
    AsChangeset, Debug, Clone, PartialEq)]
#[table_name = "books"]
pub struct NewBook { 
    pub id: Option<i32>, 
    pub title: Option<String>,
    pub genre: Option<String>,
    pub user_id: Option<i32>
}