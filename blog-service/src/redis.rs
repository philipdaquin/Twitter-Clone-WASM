use redis::{Client, RedisError};

pub async fn create_client(redis_url: String) -> Result<Client, RedisError> { 
    Ok(Client::open(redsi_url))?
}