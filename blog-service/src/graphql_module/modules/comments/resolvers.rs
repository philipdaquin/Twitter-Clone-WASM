use super::providers::{self, *};
use async_graphql::*;
use async_graphql::Error;
use super::models::*;
use crate::graphql_module::context::get_conn_from_ctx;

#[derive(Default)]
pub struct CommentQuery;

// #[Object]
impl CommentQuery { 
    async fn get_all_comments(&self, ctx: &Context<'_>) -> Result<Vec<CommentObject>, Error> {
        let conn = get_conn_from_ctx(ctx);
        let comments = providers::get_all_comments(&conn)
            .expect("Could not get Comment")
            .iter()
            .map(CommentObject::from)
            .collect();
        Ok(comments)
    }
    async fn get_comment_by_post(&self, ctx: &Context<'_>, comment_id: ID) -> Result<Vec<CommentObject>, Error> {
        let conn = get_conn_from_ctx(ctx);
        let comment_id = comment_id
            .to_string()
            .parse::<i32>()
            .expect("");
        let comment = providers::get_comments_by_post(comment_id, &conn)
            .expect("")
            .iter()
            .map(CommentObject::from)
            .collect();

        Ok(comment)
    }
}

#[derive(Default)]
pub struct CommentMutation;

#[Object]
impl CommentMutation { 
    async fn insert_comment(&self, ctx: &Context<'_>) -> Result<CommentObject, Error> {
        todo!()

    }
    async fn update_comment(&self, ctx: &Context<'_>) -> Result<CommentObject, Error> {
        todo!()

    }
    async fn delete_comment(&self, ctx: &Context<'_>) -> Result<bool, Error> {
        todo!()

    }
}



impl From<&Comment> for CommentObject { 
    fn from(oop: &Comment) -> Self {
        Self { 
            id: oop.id.into(),
            user_id: oop.user_id.into(), 
            body: oop.body.clone(),
            created_at: oop.created_at.clone(),
            updated_at: oop.updated_at.clone()
        }
    }
}