pub mod client_config;
mod exchange_rest_proxy_builder;
pub mod resilience_registries;

use async_trait::async_trait;
use std::sync::Arc;
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

pub trait Interceptor: Send + Sync {
    fn intercept(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder;
}

#[async_trait]
pub trait RestProxyFactory: Send + Sync {
    async fn create_proxy(
        &self,
        base_url: String,
        config: ClientConfig,
        interceptors: Vec<Arc<dyn Interceptor>>,
    ) -> Box<dyn std::any::Any + Send + Sync>;
}

pub struct DefaultProxyFactory;
#[async_trait]
impl RestProxyFactory for DefaultProxyFactory {
    async fn create_proxy(
        &self,
        base_url: String,
        config: ClientConfig,
        interceptors: Vec<Arc<dyn Interceptor>>,
    ) -> Box<dyn std::any::Any + Send + Sync> {
        // core 不知道具体交易所，返回 Arc<dyn Any>
        unimplemented!("交易所模块需要 downcast");
    }
}
