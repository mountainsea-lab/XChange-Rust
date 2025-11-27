pub mod client_config;
mod exchange_rest_proxy_builder;
pub mod resilience_registries;

use crate::client::client_config::ClientConfig;
use async_trait::async_trait;
use futures::FutureExt;
use futures::future::BoxFuture;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::time::sleep;

/// Retry configuration
#[derive(Clone, Debug)]
pub struct RetryConfig {
    pub max_attempts: usize,
    pub initial_delay: Duration,
    pub multiplier: f64,
}

impl RetryConfig {
    pub fn delay_for_attempt(&self, attempt: usize) -> Duration {
        self.initial_delay
            .mul_f64(self.multiplier.powi(attempt as i32))
    }
}

/// Async rate limiter
#[derive(Clone)]
pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
    capacity: u32,
    refill_period: Duration,
}
impl RateLimiter {
    /// 创建限流器
    pub fn new(capacity: u32, refill_period: Duration) -> Arc<Self> {
        let limiter = Arc::new(Self {
            semaphore: Arc::new(Semaphore::new(capacity as usize)),
            capacity,
            refill_period,
        });

        // 启动后台 refill
        RateLimiter::start_refill(limiter.clone());
        limiter
    }

    /// 获取一个 permit
    pub async fn acquire(&self) {
        self.semaphore.acquire().await.unwrap().forget();
    }

    /// 后台 refill
    fn start_refill(limiter: Arc<Self>) {
        tokio::spawn(async move {
            loop {
                sleep(limiter.refill_period).await;
                limiter.semaphore.add_permits(limiter.capacity as usize);
            }
        });
    }
}

/// ====================
/// ResilientCall 封装
/// ====================
pub struct ResilientCall<T> {
    func: Arc<
        dyn Fn() -> BoxFuture<'static, Result<T, Box<dyn std::error::Error + Send + Sync>>>
            + Send
            + Sync,
    >,
    retry_config: Option<RetryConfig>,
    rate_limiter: Option<Arc<RateLimiter>>,
}

impl<T: Send + 'static> ResilientCall<T> {
    /// 构造一个新的 ResilientCall
    pub fn new<F, Fut>(func: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<T, Box<dyn std::error::Error + Send + Sync>>>
            + Send
            + 'static,
    {
        Self {
            func: Arc::new(move || func().boxed()),
            retry_config: None,
            rate_limiter: None,
        }
    }

    /// 添加 retry 配置
    pub fn with_retry(mut self, config: RetryConfig) -> Self {
        self.retry_config = Some(config);
        self
    }

    /// 添加 rate limiter
    pub fn with_rate_limiter(mut self, limiter: Arc<RateLimiter>) -> Self {
        self.rate_limiter = Some(limiter);
        self
    }

    /// 执行调用
    pub async fn call(&self) -> Result<T, Box<dyn std::error::Error + Send + Sync>> {
        let retry_cfg = self.retry_config.clone();
        let func = self.func.clone();
        let limiter = self.rate_limiter.clone();

        let max_attempts = retry_cfg.as_ref().map(|c| c.max_attempts).unwrap_or(1);

        let mut last_err: Option<Box<dyn std::error::Error + Send + Sync>> = None;

        for attempt in 0..max_attempts {
            if let Some(ref rl) = limiter {
                rl.acquire().await;
            }

            match func().await {
                Ok(v) => return Ok(v),
                Err(e) => last_err = Some(e),
            }

            if let Some(cfg) = &retry_cfg {
                if attempt + 1 < max_attempts {
                    let delay = cfg.delay_for_attempt(attempt);
                    sleep(delay).await;
                }
            }
        }

        Err(last_err.expect("ResilientCall called zero times"))
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
