use std::sync::Mutex;

use actix_web::{web::Data, HttpResponse};
use actix_web_lab::__reexports::tokio;
use redis::{Client, RedisError, ToRedisArgs, Connection, RedisResult};
use super::graphql_module::posts::resolver::PostObject;
use super::error::ServiceError;
use crate::graphql_module::posts::models::NEW_POST_USER;


/// Connect to Redis 
pub async fn create_client(host_addr: String) -> Result<Client, RedisError> { 
    Ok(Client::open(host_addr)?)
}

/// Write the Value into a vector of bytes, in this case, we are caching a Post
/// and turning into a string so we can use it as an argument for K-V pair
/// 'write_redis_args', each item is a single argument
impl ToRedisArgs for PostObject { 
    fn write_redis_args<W>(&self, out: &mut W)
    where
            W: ?Sized + redis::RedisWrite {
        out.write_arg_fmt(serde_json::to_string(self)
            .expect("Cannot Serialize PostObject as String")
        )   
    }
}

//  Redis Pub/ Sub
//  Senders are not programmed to send their messages to specific receivers 
//  Rather, they will publish messages irrespectively without having the
//  knowledge of what subscribers there may be 
pub async fn start_pubsub(client: &Client) -> Result<(), ServiceError> { 
    let mut pub_sub = client.get_async_connection().await?.into_pubsub();
    // pub_sub.subscribe(NEW_POST_USER).await?;

    // tokio::spawn(async move { 
    //     while let Some(msg) = pub_sub.on_message().next().await { 
            
    //     }
    // })


    todo!()
}