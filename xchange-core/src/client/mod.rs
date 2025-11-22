pub mod client_config;
pub mod resilience_registries;

use std::time::Duration;

/// RetryConfig + backoff small impl
#[derive(Clone)]
pub struct RetryConfig {
    pub max_attempts: usize,
    pub initial_delay: Duration,
    pub multiplier: u64,
}

impl RetryConfig {
    pub fn delay_for_attempt(&self, attempt: usize) -> Duration {
        let factor = self.multiplier.saturating_pow(attempt as u32);
        self.initial_delay * factor as u32
    }
}

use crate::client::client_config::ClientConfig;
use tokio::time::{Instant, sleep};

#[derive(Clone)]
pub struct RateLimiter {
    capacity: u32,
    tokens: u32,
    refill_period: Duration,
    last_refill: Instant,
}

impl RateLimiter {
    pub fn new(capacity: u32, period: Duration) -> Self {
        Self {
            capacity,
            tokens: capacity,
            refill_period: period,
            last_refill: Instant::now(),
        }
    }

    pub async fn acquire(&mut self) {
        loop {
            // refill tokens
            let now = Instant::now();
            if now.duration_since(self.last_refill) >= self.refill_period {
                self.tokens = self.capacity;
                self.last_refill = now;
            }

            if self.tokens > 0 {
                self.tokens -= 1;
                return;
            }

            sleep(Duration::from_millis(10)).await;
        }
    }
}

pub trait ClientConfigCustomizer {
    fn customize(&self, config: &mut ClientConfig);
}
