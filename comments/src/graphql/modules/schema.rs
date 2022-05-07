use std::sync::Mutex;

use async_graphql::*;
use async_graphql_actix_web::*;
use chrono::NaiveDateTime;
use futures::{Stream, StreamExt};
use rdkafka::Message;
use rdkafka::producer::FutureProducer;
use serde::{Serialize, Deserialize};
use super::{resolver, model::NewComment};
use crate::graphql::config::get_conn_from_ctx;
use crate::graphql::kafka;


#[derive(Serialize, Deserialize)]
pub struct User { 
    pub id: ID
}
#[Object(extends)]
impl User { 
    #[graphql(external)]
    pub async fn id(&self, id: ID) -> User { 
        User { id }
    }
    #[graphql(name = "getAllUserComments")]
    pub async fn comments(&self, ctx: &Context<'_>, user_id: ID) -> Vec<CommentType> { 
        resolver::get_comments_by_user(parse_id(user_id),&get_conn_from_ctx(ctx))
            .expect("")
            .iter()
            .map(|f| CommentType::from(f))
            .collect()
    }
    #[graphql(name = "getUserCommentFromPost")]
    pub async fn get_user_comment_post(&self, ctx: &Context<'_>, post_id: ID, user_id: ID) -> FieldResult<Vec<CommentType>> { 
        let user_comments = resolver::get_comment_of_user(parse_id(post_id), parse_id(user_id), &get_conn_from_ctx(ctx))
            .expect("")
            .iter()
            .map(|f| CommentType::from(f))
            .collect();
        Ok(user_comments)
    }
}
#[derive(Serialize, Deserialize)]
pub struct PostObject { 
    pub id: ID
}
#[Object(extends)]
impl PostObject { 
    #[graphql(external)]
    pub async fn id(&self, id: ID) -> PostObject { 
        PostObject { id }
    }
    #[graphql(name = "getAllCommentsFromPost")]
    pub async fn comments(&self, ctx: &Context<'_>, post_id: ID) -> Vec<CommentType> { 
        resolver::get_comments_by_post(parse_id(post_id), &get_conn_from_ctx(ctx))
            .expect("")
            .iter()
            .map(|f| CommentType::from(f))
            .collect()
    }
}

#[derive(Default)]
pub struct CommentQuery;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct CommentType { 
    pub id: ID,
    pub author_id: User,
    pub post_id: PostObject,
    pub body: String,
    pub media: String, 
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}

#[Object]
impl CommentQuery {
    /// Insert all the keys referencing to Post and User 
    #[graphql(entity)]
    pub async fn get_user_details(&self, #[graphql(key)] id: ID) -> User { 
        User { id }
    }
    pub async fn get_post_details(&self, #[graphql(key)] id: ID) -> PostObject { 
        PostObject { id }
    }
    #[graphql(name = "getAllComments")]
    pub async fn get_all(&self, ctx: &Context<'_>) -> Option<Vec<CommentType>> { 
        let comments = resolver::get_all_comments(&get_conn_from_ctx(ctx))
            .expect("")
            .iter()
            .map(|f| CommentType::from(f))
            .collect();
        Some(comments)
    }
    #[graphql(name = "getCommentsFromPost")]
    pub async fn get_comments_from_post(&self, ctx: &Context<'_>, post_id: ID) -> Vec<CommentType> { 
        resolver::get_comments_by_post(parse_id(post_id), &get_conn_from_ctx(ctx))
            .expect("")
            .iter()
            .map(|f| CommentType::from(f))
            .collect()
    }
    #[graphql(name = "getComment")]
    pub async fn get_comment(&self, ctx: &Context<'_>, comment_id: ID) -> Option<CommentType> { 
        resolver::get_comments_by_id(parse_id(comment_id), &get_conn_from_ctx(ctx))
            .ok()
            .map(|f| CommentType::from(&f))
    }
}
#[derive(Default)]
pub struct CommentMutation;

#[derive(InputObject, Deserialize, Serialize)]
pub struct CommentInput { 
    pub author_id: ID, 
    pub post_id: ID, 
    pub body: String, 
    pub media: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}

#[Object]
impl CommentMutation { 
    #[graphql(name = "createComment")]
    pub async fn create_comment(&self, ctx: &Context<'_>, new_comment: CommentInput) -> FieldResult<CommentType> {
        let new_comment = resolver::create_comment(
        NewComment::from(&new_comment), 
        &get_conn_from_ctx(ctx))
            .expect("");
        let comment = CommentType::from(&new_comment);

        let kafka_producer = ctx
            .data::<FutureProducer>()
            .expect("Cannot Access Kafka Producer");
        let kafka_message = serde_json::to_string(&comment)
            .expect("Cannot Serialize Comment");
        kafka::send_message(kafka_producer, kafka_message).await;

        Ok(comment)
    }
    #[graphql(name = "updateComment")]
    pub async fn update_comment(&self, ctx: &Context<'_>, post_id: ID, author_id: ID, new_comment: CommentInput) -> Option<CommentType> {
        let comment = resolver::update_user_comment(
            parse_id(post_id), 
            parse_id(author_id), 
            NewComment::from(&new_comment), 
            &get_conn_from_ctx(ctx)
        ).expect("");
        Some(CommentType::from(&comment))
    }
    #[graphql(name = "deleteComment")]
    pub async fn delete_comment(
        &self, 
        ctx: &Context<'_>, 
        comment_id: ID,
        post_id: ID,
        author_id: ID) -> FieldResult<bool> {
        
        resolver::delete_comment(
            parse_id(comment_id),
            parse_id(post_id),
            parse_id(author_id),
            &get_conn_from_ctx(ctx)
        )?;
        Ok(true)
            
    }
}

#[derive(Default)]
pub struct CommentSubscription;

#[Subscription]
impl CommentSubscription { 
    pub async fn latest_comments<'ctx>(&self, ctx: &'ctx Context<'_>) -> impl Stream<Item = CommentType> + 'ctx { 
        let kafka_consumer_counter = ctx.data::<Mutex<i32>>().expect("Cannot get Kafka Consumer");
        let consumer_groud_id = kafka::get_kafka_consumer_id(kafka_consumer_counter);
        let consumer = kafka::create_consumer(consumer_groud_id);

        async_stream::stream! { 
            let mut stream = consumer.stream();
            while let Some(val) = stream.next().await { 
                yield match val { 
                    Ok(message) => { 
                        let payload = message.payload().expect("");
                        let message = String::from_utf8_lossy(payload).to_string();

                        serde_json::from_str(&message)
                            .expect("Cannot Deserialize Comments")
                    }
                    Err(e) => panic!("Error while Kafka Message Processing: {}", e)
                };
            }
        }
    }
}


fn parse_id(id: ID) -> i32 { 
    id.parse::<i32>().expect("")
}

