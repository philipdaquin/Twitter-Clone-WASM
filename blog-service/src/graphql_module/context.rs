
use std::sync::{Mutex, Arc};
use super::posts::resolver::Subscription;
use actix_cors::Cors;
use actix_web::{get, middleware::Logger, Error,  route, web::{self}, App, HttpServer, Responder, HttpRequest, HttpResponse, guard};
use actix_web_lab::respond::Html;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema, Context, extensions::ApolloTracing
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use crate::{db::{DbPool, DbPooledConnection}, utils::redis::create_connection};
use super::schema::{Mutation, Query, AppSchema, AppSchemaBuilder};
use diesel::{result::Error as DbError, QueryDsl};
use diesel_migrations::{MigrationError, embed_migrations};
use common_utils::token::get_role;
use crate::utils::{kafka::create_producer, error::ServiceError};
use redis::{aio::ConnectionManager as RedisManager, Client as RedisClient, aio::Connection as RedisConnection};
use crate::utils::rate_limiter::RateLimiter;


pub fn configure_service(cfg: &mut web::ServiceConfig) { 
    cfg
    .service(graphql)
    .service(graphql_playground)
    .service(
        web::resource("/graphiql")
            .route(web::get()
                .guard(guard::Header("upgrade", "websocket"))
                .to(index_ws)
        )
    );
}
/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(
    schema: web::Data<AppSchema>, 
    req: GraphQLRequest, 
    http: HttpRequest,
    api_limiter: web::Data<RateLimiter>
) -> Result<GraphQLResponse, ServiceError> {

    api_limiter
        .assert_rate_limiter_request(get_ip(http.clone()))
        .await
        .map_err(|_| ServiceError::BadClientData);

    let (role, mut request) = (get_role(http), req.into_inner());
    if let Some(user) = role { 
        request = request.data(user);
    }
    Ok(schema.execute(request).await.into())
}
/// GraphiQL playground UI
#[get("/graphiql")]
pub async fn graphql_playground() -> impl Responder {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
    ))
}
/// Get user IP address 
pub fn get_ip(http: HttpRequest) -> String { 
    http.connection_info()
    .realip_remote_addr()
    .expect("Unable to get User Ip")
    .to_string()
}


pub async fn index_ws(
    schema: web::Data<AppSchema>, 
    req: HttpRequest, 
    payload: web::Payload
) -> Result<HttpResponse, Error> { 
    GraphQLSubscription::new(Schema::clone(&*schema))
        .start(&req, payload)
}

embed_migrations!();

pub fn create_schema(
    pool: DbPool, 
    redis_pool: RedisClient, 
    redis_connection: RedisManager
) -> AppSchema { 
    //  SQL Database
    let arc_pool = Arc::new(pool);
    //  Kafka Queue
    let kafka_consumer = Mutex::new(0);
    // Caching Service 
    let arc_redis_connection = Arc::new(redis_connection);

    Schema::build(Query::default(), Mutation::default(), Subscription
    )
    .enable_federation()
    .data(arc_redis_connection)
    // Add a global data that can be accessed in the Schema
    //  Redis Caching Client  
    .data(redis_pool)
    //  SQL Database Pool
    .data(arc_pool)
    //  Kafka Queue
    .data(create_producer())
    .data(kafka_consumer)
    //  Apollo Tracing 
    .extension(ApolloTracing)
    //  Build Schema
    .finish()
}
pub fn run_migrations(pool: &DbPool) { 
    let conn = pool
        .get()
        .expect("Database Connection Pool - Migrations error!");
    embedded_migrations::run(&conn)
        .expect("Failed to run database migrations");
}
/// Access DBPool from Context
pub fn get_conn_from_ctx(ctx: &Context<'_>) -> DbPooledConnection { 
    ctx.data::<Arc<DbPool>>()
        .expect("Failed to get Db Pool")
        .get()
        .expect("Failed to Connect to Database")
}
/// Access Redis from the Context, use 'create_connection' to establish connection asynchronously
pub async fn get_redis_conn_from_ctx(ctx: &Context<'_>) -> RedisClient { 
    ctx.data::<RedisClient>()
        .expect("Failed to get Redis Client")
        .clone()
}
/// Access Redis Database Connection
pub async fn get_redis_conn_manager(ctx: &Context<'_>) -> RedisManager { 
    ctx.data::<RedisManager>()
        .expect("Failed to get Redis Connection Manager")
        .clone()
}
