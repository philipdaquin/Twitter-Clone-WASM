use actix_cors::Cors;
use actix_web::{get, middleware::Logger, route, web, App, HttpServer, Responder};
use actix_web_lab::respond::Html;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use crate::redis::create_client;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use crate::graphql_module::context::{graphql, graphql_playground, create_schema, run_migrations};
use crate::db::{DatabaseKind, establish_connection};
use super::graphql_module::context::configure_service;
use std::env::var;


pub async fn new_server(port: u32) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    //  PostGreSQL Database Pool
    let db_pool = establish_connection(DatabaseKind::Example);
    run_migrations(&db_pool);
    //  GraphQl Schema
    let schema = web::Data::new(create_schema(db_pool));
    //  Redis Config 
    let redis_url = std::env::var("REDIS_URL").expect("Cannot Read Redis");
    let redis_client = create_client(redis_url).await.expect("Cannto Create Redis Client, Pub/Sub");
    let redis_connection_manager = redis_client
        .get_tokio_connection_manager()
        .await
        .expect("Cannot create Redis Connection Manager");
    

    log::info!("starting HTTP server on port 8080");
    log::info!("GraphiQL playground: http://localhost:8080/graphiql");

    HttpServer::new(move || {
        App::new()
            
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