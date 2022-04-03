use async_graphql::*;
use crate::graphql_module::schema::{Mutation, Query};
use serde::{Deserialize, Serialize};
use super::{provider, models::Post};
use crate::graphql_module::context::get_conn_from_ctx;
use super::models::FormPost;
use chrono::{NaiveDateTime, Local};
use super::models::{PostInput, Posts};
use async_graphql::Error;


#[derive(Default)]
pub struct PostQuery;

#[Object]
impl PostQuery { 
    #[graphql(name = "getPosts")]
    async fn get_posts(&self, ctx: &Context<'_>) -> Vec<Posts> {
        let conn = get_conn_from_ctx(ctx);
        provider::get_all(&conn)
            .expect("Cannot get Blog posts ")
            .iter()
            .map(Posts::from)
            .collect()
    }
    #[graphql(name = "getPostbyId")]
    async fn get_post_by_id(&self, ctx: &Context<'_>, post_id: ID) -> Result<Option<Posts>, Error> { 
        let conn = get_conn_from_ctx(ctx);
        let id = post_id
            .to_string()
            .parse::<i32>()
            .expect("Could not Parse Post_ID");
        let post = provider::get_id(id, &conn)  
            .ok()
            .map(|w| Posts::from(&w));
        Ok(post)

    }
    #[graphql(name = "getPostsbyAuthor")]
    async fn get_post_by_authorid(&self, ctx: &Context<'_>, user_id: ID) -> Result<Vec<Posts>, Error> { 
        let conn = get_conn_from_ctx(ctx);
        let author_id = user_id
            .to_string()
            .parse::<i32>()
            .expect("Could not Parse Post_ID");
        let post_by = provider::get_by_author(author_id, &conn)
            .expect("Cannot get any User posts")
            .iter()
            .map(|s| Posts::from(s))
            .collect();
        Ok(post_by)
    }
}
#[derive(Default)]
pub struct PostMutation;

#[Object]
impl PostMutation { 
    #[graphql(name = "createPosts")]
    async fn create_post(&self, ctx: &Context<'_>, form: PostInput) -> Result<Posts, Error> {
        let conn = get_conn_from_ctx(ctx);
        
        let new_post = FormPost { 
            slug: Some(form.slug),
            created_at: Local::now().naive_local(),
            updated_at: None, 
            title: Some(form.title),
            description: Some(form.description),
            body: Some(form.body),
            featured_image: Some(form.featured_image)
        };
        let post = provider::create_post(new_post, &conn)
            .ok()
            .map(|e| Posts::from(&e))
            .expect("Unable to convert Post to Posts");
        Ok(post)
    }
    #[graphql(name = "updatePosts")]
    async fn update_post(
        &self, 
        ctx: &Context<'_>, 
        form: PostInput,
        post_id: ID
    ) -> Result<Posts, Error> {
        // let conn = get_conn_from_ctx(ctx);
        // let post_id = post_id
        //     .to_string()
        //     .parse::<i32>()
        //     .expect("Could not Parse POst Id to int");
        // let post = provider::update_post( form, &conn)
        //     .expect("")
        //     .map(Posts::from);
        // Ok(post)
        todo!()
    }
    #[graphql(name = "deletePosts")]
    async fn delete_post(&self, ctx: &Context<'_>) -> Result<bool, Error> { 
        Ok(true)
    }
}


#[Object]
impl Posts  { 
    async fn id(&self) -> ID { 
        self.id.clone()
    }
    async fn slug(&self) -> &str {
        &self.slug
    }
    async fn title(&self) -> &str { 
        &self.title
    }
    async fn description(&self) -> &str  {
        &self.description
    }
    async fn body(&self) -> &str { 
        &self.body
    }
    async fn image(&self) -> &str { 
        &self.featured_image
    }
}
impl From<&Post> for Posts { 
    fn from(oop: &Post) -> Self {
        Posts { 
            id: oop.id.into(),
            slug: oop.slug.clone(),
            title: oop.title.clone(),
            description: oop.description.clone(),
            body: oop.body.clone(),
            featured_image: oop.featured_image.clone()
        }
    }
}