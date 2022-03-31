use auth_service::server::new_server;
use failure::Error;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<(), Error> { 
    dotenv().ok();
    let port = std::env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u32>().ok())
        .unwrap_or(3000);
        
    new_server(port)
        .await
        .map_err(Into::into)
}