use std::sync::Mutex;

use async_graphql::*;
use crate::{graphql_module::schema::{Mutation, Query}, kafka};
use serde::{Deserialize, Serialize};
use super::{provider, models::Post};
use crate::graphql_module::context::get_conn_from_ctx;
use super::models::FormPost;
use chrono::{NaiveDateTime, Local};
use async_graphql::Error;

#[derive(Default)]
pub struct PostQuery;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct PostObject { 
    pub id: ID,
    pub user_id: User,
    pub slug: String, 
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime, 
    pub title: String,
    pub description: String, 
    pub body: String, 
    pub featured_image: String
}

#[Object]
impl PostQuery { 
    /// Resolver Reference 
    #[graphql(entity)]
    pub async fn get_user_details(&self, #[graphql(key)] id: ID) -> User { 
        User { 
            id
        }
    }
    #[graphql(entity)]
    pub async fn get_post_details(
        &self, 
        ctx: &Context<'_>, 
        #[graphql(key)] id: ID
    ) -> Option<PostObject> { 
        get_post_detail(ctx, id)
    }

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
    pub async fn get_post_by_id(&self, ctx: &Context<'_>, post_id: ID) -> Option<PostObject> { 
        get_post_detail(ctx, post_id)

    }
    #[graphql(name = "getPostsbyAuthor")]
    async fn get_post_by_authorid(&self, ctx: &Context<'_>, user_id: ID) -> Vec<PostObject> { 
       get_posts_user(ctx, user_id)
    }
}
/// Gets the Post Information by using the Post Id
pub fn get_post_detail(ctx: &Context<'_>, post_id: ID) -> Option<PostObject> { 
    provider::get_post_by_id(parse_id(post_id), &get_conn_from_ctx(ctx))
        .ok()
        .map(|f| PostObject::from(&f))
}
/// Gets the Post under the author: UserId
pub fn get_posts_user(ctx: &Context<'_>, user_id: ID) -> Vec<PostObject> { 
    provider::get_by_posts_by_author(
        parse_id(user_id), &get_conn_from_ctx(ctx)
    )
        .expect("Cannot get any User Posts")
        .iter()
        .map(|s| PostObject::from(s))
        .collect()
}
#[derive(Serialize, Deserialize)]
pub struct User { 
    pub id: ID
}
#[Object(extends)]
impl User { 
    /// The Key needed for 
    #[graphql(external)]
    pub async fn id(&self, id: ID) -> User { 
        User { id }
    }
    /// Load all Posts under User 
    #[graphql(name = "getPostsByUser")]
    pub async fn get_user_posts(&self, ctx: &Context<'_>, id: ID) -> Vec<PostObject> { 
        get_posts_user(ctx, id)
    }
}


#[derive(Default)]
pub struct PostMutation;

#[derive(InputObject)]
pub struct PostInput { 
    pub user_id: ID,
    pub slug: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub title: Option<String>, 
    pub description: Option<String>, 
    pub body: Option<String>,
    pub featured_image: Option<String> 
} 
#[Object]
impl PostMutation { 
    #[graphql(name = "createPost")]
    async fn create_post(&self, ctx: &Context<'_>, form: PostInput) -> Result<PostObject, Error> {
        let post = provider::create_post(FormPost::from(&form), &get_conn_from_ctx(ctx))?;
        //  In the mutation, post creation a messgage is sent to the kafka.
        let producer = ctx.data::<FutureProducer>().expect("Cannot get Kafka Producer");
        let message = serde_json::to_string(&PostObject::from(&post)).expect("Cannot serialize a post");
        kafka::send_message(producer, message).await;

        Ok(PostObject::from(&post))
    }
    #[graphql(name = "updatePosts")]
    async fn update_post(
        &self, 
        ctx: &Context<'_>, 
        form: PostInput,
        post_id: ID,
        user_id: ID
    ) -> Result<PostObject, Error> {
        //  Convert the grahql input into readable database input
        let new_post = provider::update_post(
            parse_id(post_id), 
            parse_id(user_id), 
            FormPost::from(&form), 
            &get_conn_from_ctx(ctx)
        ).expect("");
        //  Convert Post (from the database), into Graphql object
        Ok(PostObject::from(&new_post))

    }
    #[graphql(name = "deletePosts")]
    async fn delete_post(&self, ctx: &Context<'_>, post_author: i32, post_id: i32 ) -> Result<bool, Error> { 
        let conn = get_conn_from_ctx(ctx);
        provider::delete_post(post_author, post_id, &conn)
            .expect("Couldn't delete Post");
        Ok(true)
    }
}


//  Get the latest Posts
//  Subscriptions
use rdkafka::{producer::FutureProducer, Message};
use futures::{Stream, StreamExt};
use crate::kafka::{create_consumer, create_producer, get_kafka_consumer_id, send_message};
pub struct Subscription;


//  The API client can be notified of the event by a subscription that listens to Kafka consumer
#[Subscription]
impl Subscription { 
    async fn latest_post<'ctx> (
        &self,
        ctx: &'ctx Context<'_>,
    ) -> impl Stream<Item = PostObject> + 'ctx { 
        let kafka_consumer = ctx
            .data::<Mutex<i32>>().expect("Cnnot get the Kafka Consumer counter");
        let consumer_id = get_kafka_consumer_id(kafka_consumer);
        let consumer = create_consumer(consumer_id);
        // stream! macros returns an anonymous type implementing the Stream trait. 
        async_stream::stream! {
            let mut stream = consumer.stream();
            while let Some(val) = stream.next().await { 
                yield match val { 
                    Ok(msg) => { 
                        let payload = msg.payload().expect("Kafka msg should contain payload");
                        let msg = String::from_utf8_lossy(payload).to_string();
                        serde_json::from_str(&msg).expect("Cannot Deserialize a Message")
                    }
                    Err(e) => panic!("Error while Kafka message processing: {}", e)
                };
            }
        }
    }
}
//  Helper Parser 
pub fn parse_id(id: ID) -> i32 { 
    id.parse::<i32>().expect("ParseIntError")
}