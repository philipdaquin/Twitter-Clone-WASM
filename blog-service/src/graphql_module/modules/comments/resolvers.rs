use super::providers::*;
use async_graphql::*;
use async_graphql::Error;
use super::models::*;


#[derive(Default)]
pub struct CommentQuery;

#[Object]
impl CommentQuery { 
    async fn get_all_comments(&self, ctx: &Context<'_>) -> Result<Vec<CommentObject>, Error> {
        todo!()
    }
    async fn get_comment_by_post(&self, ctx: &Context<'_>) -> Result<CommentObject, Error> {
        todo!()
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