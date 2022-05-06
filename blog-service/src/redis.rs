use redis::{Client, RedisError};


/// Create Client 
pub async fn create_client(redis_url: String) -> Result<Client, RedisError> { 
    Ok(Client::open(redis_url)?)
}
