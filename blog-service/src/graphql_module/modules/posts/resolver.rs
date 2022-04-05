use async_graphql::*;
use crate::graphql_module::schema::{Mutation, Query};
use serde::{Deserialize, Serialize};
use super::{provider, models::Post};
use crate::graphql_module::context::get_conn_from_ctx;
use super::models::FormPost;
use chrono::{NaiveDateTime, Local};
use super::models::{PostInput, PostObject};
use async_graphql::Error;
use rdkafka::{producer::FutureProducer, Message};

#[derive(Default)]
pub struct PostQuery;

#[Object]
impl PostQuery { 
    #[graphql(name = "getPost")]
    async fn get_post(&self, ctx: &Context<'_>) -> Vec<PostObject> {
        let conn = get_conn_from_ctx(ctx);
        provider::get_all(&conn)
            .expect("Cannot get Blog PostObject ")
            .iter()
            .map(PostObject::from)
            .collect()
    }
    #[graphql(name = "getPostbyId")]
    async fn get_post_by_id(&self, ctx: &Context<'_>, post_id: ID) -> Result<Option<PostObject>, Error> { 
        let conn = get_conn_from_ctx(ctx);
        let id = post_id
            .to_string()
            .parse::<i32>()
            .expect("Could not Parse Post_ID");
        let post = provider::get_id(id, &conn)  
            .ok()
            .map(|w| PostObject::from(&w));
        Ok(post)

    }
    #[graphql(name = "getPostsbyAuthor")]
    async fn get_post_by_authorid(&self, ctx: &Context<'_>, user_id: ID) -> Result<Vec<PostObject>, Error> { 
        let conn = get_conn_from_ctx(ctx);
        let author_id = user_id
            .to_string()
            .parse::<i32>()
            .expect("Could not Parse Post_ID");
        let post_by = provider::get_by_author(author_id, &conn)
            .expect("Cannot get any User Posts")
            .iter()
            .map(|s| PostObject::from(s))
            .collect();
        Ok(post_by)
    }
}
#[derive(Default)]
pub struct PostMutation;

#[Object]
impl PostMutation { 
    #[graphql(name = "createPost")]
    async fn create_post(&self, ctx: &Context<'_>, form: PostInput) -> Result<PostObject, Error> {
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
            .map(|e| PostObject::from(&e))
            .expect("Unable to convert Post to PostsObject");
        Ok(post)
    }
    #[graphql(name = "updatePosts")]
    async fn update_post(
        &self, 
        ctx: &Context<'_>, 
        form: PostInput,
        post_id: ID
    ) -> Result<PostObject, Error> {
        // let conn = get_conn_from_ctx(ctx);
        // let post_id = post_id
        //     .to_string()
        //     .parse::<i32>()
        //     .expect("Could not Parse POst Id to int");
        // let post = provider::update_post( form, &conn)
        //     .expect("")
        //     .map(PostObject::from);
        // Ok(post)
        todo!()
    }
    #[graphql(name = "deletePosts")]
    async fn delete_post(&self, ctx: &Context<'_>, post_author: i32, post_id: i32 ) -> Result<bool, Error> { 
        let conn = get_conn_from_ctx(ctx);
        provider::delete_post(post_author, post_id, &conn)
            .expect("Couldn't delete Post");
        Ok(true)
    }
}

#[Object]
impl PostObject  { 
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

impl From<&Post> for PostObject { 
    fn from(oop: &Post) -> Self {
        PostObject { 
            id: oop.id.into(),
            slug: oop.slug.clone(),
            title: oop.title.clone(),
            description: oop.description.clone(),
            body: oop.body.clone(),
            featured_image: oop.featured_image.clone()
        }
    }
}

//  Get the latest Posts
pub struct Subscription;

#[Subscription]
impl Subscription { 
    async fn latest_post<'ctx> { 
        &self,
        ctx: &'ctx Cntext<'_>,
    } -> impl Stream<Item = PostObject> + 'ctx { 
        

    }
}