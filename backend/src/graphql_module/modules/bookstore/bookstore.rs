use async_graphql::*;
use crate::{graphql_module::context::get_conn_from_ctx, db::DbPool};
use super::{provider::{get_book_by_user_id, get_all, get_book, self}, models::NewBook};
use actix_web::error as ServerError;
use super::models::BookEntity;


#[derive(Default)]
pub struct BookQuery;

#[derive(Default)]
pub struct BookMutation;


#[derive(SimpleObject)]
struct Books  {
    pub id: ID, 
    pub title: String,
    pub genre: String, 
    pub user_id: i32
}

struct Author { 
    user_id: ID
}

#[Object]
impl BookQuery { 
    async fn get_alls(&self, ctx: &Context<'_>) -> FieldResult<Vec<Books>> { 
        let res = provider::get_all(&get_conn_from_ctx(ctx)) 
            .expect("a")
            .iter()
            .map(Books::from)
            .collect::<Vec<Books>>();
        Ok(res)
    } 
    async fn get_books(&self, ctx: &Context<'_>, id: ID) -> Option<Books> { 
        let id = id
            .to_string()
            .parse::<i32>()
            .expect("Parse Error");
        provider::get_book(id, &get_conn_from_ctx(ctx))
            .ok()
            .map(|e| Books::from(&e))

    } 
    #[graphql(entity)]
    async fn get_book_by_user_ids(&self, id: ID) -> Author { 
        Author { user_id: id}
    }
}
// #[Object]
// impl BookMutation { 
//     async fn create_book(&self, ctx: &Context<'_>, new_book: NewBook) -> Result<Books, Error>  {
//         let new = NewBook { id: todo!(), title: todo!(), genre: todo!(), user_id: todo!() };
//         todo!()
//     }
// }



#[Object(extends)]
impl Author { 
    async fn user_ids(&self) -> &ID { 
        &self.user_id
    }
    async fn books(&self, ctx: &Context<'_>) -> Vec<Books> { 
        let id = self.user_id.to_string().parse::<i32>().expect("Failed to Parse User_ID");
        provider::get_book_by_user_id(id, &get_conn_from_ctx(ctx))
            .expect("Cannot get Books from an Author")
            .iter()
            .map(Books::from)
            .collect()
    }
}

impl From<&BookEntity> for Books { 
    fn from(book: &BookEntity) -> Self { 
        Self { 
            id: book.id.into(),
            title: book.title.clone(),
            genre: book.genre.clone(),
            user_id: book.user_id.into()
        }
    }
}

