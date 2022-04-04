use super::providers::{self, *};
use async_graphql::*;
use async_graphql::Error;
use super::models::*;
use crate::graphql_module::context::get_conn_from_ctx;
use common::token::Role as AuthRole;
use chrono::{NaiveDateTime, Local};


#[derive(Default)]
pub struct CommentQuery;

#[Object]
impl CommentQuery { 
    #[graphql(name = "getComments")]
    pub async fn get_all_comments(&self, ctx: &Context<'_>) -> Result<Vec<CommentObject>, Error> {
        let conn = get_conn_from_ctx(ctx);
        let comments = providers::get_all_comments(&conn)
            .expect("Could not get Comment")
            .iter()
            .map(CommentObject::from)
            .collect::<Vec<_>>();

        Ok(comments)
    }
    
    #[graphql(name = "getCommentByPost")]
    pub async fn get_comment_by_post(&self, ctx: &Context<'_>, comment_id: ID) -> Result<Vec<CommentObject>, Error> {
        let conn = get_conn_from_ctx(ctx);
        let comment_id = comment_id
            .to_string()
            .parse::<i32>()
            .expect("Could not parse into i32");

        let comment = providers::get_comments_by_post(comment_id, &conn)
            .expect("Could not get list of CommentObjects")
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
    #[graphql(name = "createPost")]
    async fn insert_comment(&self, ctx: &Context<'_>, user_id: ID, form: NewComment) -> Result<Option<CommentObject>, Error> {
        let conn = get_conn_from_ctx(ctx);
        
        let new_comment = CommentInput { 
            post_id: form.post_id
                .to_string()
                .parse::<i32>()
                .expect(""),
            user_id: user_id
                .to_string()
                .parse::<i32>()
                .expect(""),
            body: form.body,
            created_at: Local::now().naive_local(),
            updated_at: None
        };
        let comment = providers::add_comment(new_comment, &conn)
            .ok()
            .map(|x| CommentObject::from(&x));
        
        Ok(comment)
    }

    #[graphql(name = "getPost")]
    async fn update_comment(&self, ctx: &Context<'_>) -> Result<CommentObject, Error> {
        let conn = get_conn_from_ctx(ctx);


    }

    #[graphql(name = "getPost")]
    async fn delete_comment(&self, ctx: &Context<'_>) -> Result<bool, Error> {
        let conn = get_conn_from_ctx(ctx);


    }
}



impl From<&Comment> for CommentObject { 
    fn from(oop: &Comment) -> Self {
        Self { 
            id: oop.id.into(),
            user_id: oop.user_id.into(), 
            post_id: oop.post_id.into(),
            body: oop.body.clone(),
            created_at: oop.created_at.clone(),
            updated_at: oop.updated_at.clone()
        }
    }
}