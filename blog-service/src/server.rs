use actix_cors::Cors;
use actix_web::{get, middleware::Logger, route, web, App, HttpServer, Responder};
use actix_web_lab::respond::Html;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use redis::aio::ConnectionManager;
use crate::utils::{redis::{start_pubsub, create_client, create_connection, RedisDatabase}, error::ServiceError, };
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use crate::graphql_module::context::{graphql, graphql_playground, create_schema, run_migrations};
use crate::db::{DatabaseKind, establish_connection};
use super::graphql_module::context::configure_service;
use std::env::var;
use crate::utils::rate_limiter::RateLimiter;


pub async fn new_server(port: u32) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    //  PostGreSQL Database Pool
    let db_pool = establish_connection(DatabaseKind::Example);
    run_migrations(&db_pool);
    //  Create a Redis Client `
    let redis_client = create_client(RedisDatabase::Example)
        .await
        .expect("Unable to create Redis Client Connection");
    let redis_connection_manager = redis_client
        .get_tokio_connection_manager()
        .await
        .expect("Cannot Create Redis Connection Manager");
    //  GraphQl Schema
    let schema = web::Data::new(create_schema(
        db_pool, 
        redis_client.clone(), 
        redis_connection_manager.clone()));
    //  In Memory API Limiter 
    let redis_api_limiter = web::Data::new(RateLimiter::new(redis_connection_manager));
    //  Redis Config 
    // let redis_connection_manager = redis_client
    //     .get_tokio_connection_manager()
    //     .await
    //     .expect("Cannot create Redis Connection Manager");
    start_pubsub(&redis_client)
        .await
        .expect("Unable to start Redis Pub/ Sub");

    log::info!("starting HTTP server on port 8080");
    log::info!("GraphiQL playground: http://localhost:8080/graphiql");

    HttpServer::new(move || {
        App::new()
            .app_data(redis_api_limiter.clone())
            .app_data(schema.clone())
            .configure(configure_service)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}