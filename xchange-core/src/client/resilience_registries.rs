use crate::client::{RateLimiter, RetryConfig};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

pub struct ResilienceRegistries {
    pub retry_configs: HashMap<String, RetryConfig>,
    pub rate_limiters: HashMap<String, Arc<RateLimiter>>,
}

impl ResilienceRegistries {
    pub fn new() -> Self {
        let mut retry_configs = HashMap::new();

        retry_configs.insert(
            "global".into(),
            RetryConfig {
                max_attempts: 3,
                initial_delay: Duration::from_millis(50),
                multiplier: 4.0,
            },
        );

        retry_configs.insert(
            "non_idempotent".into(),
            RetryConfig {
                max_attempts: 1,
                initial_delay: Duration::from_millis(50),
                multiplier: 1.0,
            },
        );

        let mut rate_limiters = HashMap::new();
        rate_limiters.insert(
            "global".into(),
            RateLimiter::new(1200, Duration::from_secs(60)),
        );

        Self {
            retry_configs,
            rate_limiters,
        }
    }
}
