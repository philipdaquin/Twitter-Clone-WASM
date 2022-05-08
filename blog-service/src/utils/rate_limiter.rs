use redis::aio::ConnectionManager;
use crate::utils::redis as utils;
use chrono::{Utc, Timelike};

use super::error::ServiceError;
lazy_static! { 
    static ref RATE_LIMITER_KEY: &'static str = "LIMITER";
    static ref MAX_REQUEST_PER_MIN: u64 = 2;
    static ref MAX_ACCOUNTS_PER_ACCOUNT: u64 = 10;
}
/// System Design: Accurately limit excessive request using in-memory cache like Redis
/// in memory caching is fast and supports timebased expiration strategy
pub struct RateLimiter { 
    pub redis_manager: ConnectionManager
}

impl RateLimiter { 
    pub fn new(redis_manager: ConnectionManager) -> Self { 
        Self {
            redis_manager
        }
    }

    /// user can write no more than 2 posts per second 
    pub async fn assert_rate_limiter_request(&self, ip_address: String) -> Result<(), ServiceError>  {
        let time_now = Utc::now().time().minute(); 
        let rate_limiter_key = format!("{}:{}:{}", 
            RATE_LIMITER_KEY.to_string().as_str(), 
            &ip_address, 
            time_now
        );
        
        let (counter, _) : (u64, u64) = redis::pipe()
            .atomic()
            //  increase the stored counter by 1
            .incr(&rate_limiter_key, 1)
            //  set the timeout for the counter, once it reaches expiration time, delete the counter
            .expire(time_now, 60)
            .query_async(&mut self.redis_manager.clone())
            .await
            .expect("Unable to execute on Redis Service");
        
        // if the counter is larger than the limit, the request is disallowed 
        if counter >= *MAX_REQUEST_PER_MIN { 
            Err(ServiceError::Timeout)
        } else { 
            Ok(())
        }
    }
    /// Ensure the user can only create a maximum of 10 accounts per day from the same IP address
    pub async fn assert_num_accounts_created(&self, ip_address: String) -> Result<(), ServiceError> { 
        let time_now = Utc::now().hour();
        let rate_limiter_key = format!("{}:{}:{}", MAX_ACCOUNTS_PER_ACCOUNT.to_string().as_str(), ip_address, time_now);
        
        let (max_accounts, _): (u64, u64)= redis::pipe()
            .atomic()
            //  Increment by 1 
            .incr(&rate_limiter_key, 1)
            //  Expire after a day
            .expire(time_now, 86_400)
            .query_async(&mut self.redis_manager.clone())
            .await
            .expect("Unable to execute on Redis Service");
        if max_accounts > *MAX_ACCOUNTS_PER_ACCOUNT { 
            Err(ServiceError::InternalError)
        } else { 
            Ok(())
        }
    }

}