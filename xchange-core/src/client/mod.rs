pub mod client_config;
mod exchange_rest_proxy_builder;
use futures::FutureExt;
use futures::future::BoxFuture;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::Semaphore;
use tokio::time::sleep;

/// ========================
/// Retry Config
/// ========================
#[derive(Debug)]
pub struct RetryConfig {
    pub max_attempts: usize,
    pub initial_delay: Duration,
    pub multiplier: f64,
}

impl RetryConfig {
    pub fn delay_for_attempt(&self, attempt: usize) -> Duration {
        let factor = self.multiplier.powi(attempt as i32);
        self.initial_delay.mul_f64(factor)
    }
}

/// ========================
/// RateLimiter
/// ========================
#[derive(Debug)]
pub struct RateLimiter {
    capacity: usize,
    semaphore: Arc<Semaphore>,
    refill_period: Duration,
}

#[derive(Debug)]
pub enum RateLimiterError {
    AcquireFailed,
}

impl RateLimiter {
    pub fn new(capacity: usize, refill_period: Duration) -> Self {
        Self {
            capacity,
            semaphore: Arc::new(Semaphore::new(capacity)),
            refill_period,
        }
    }

    pub async fn acquire(&self) -> Result<RateLimiterPermit, RateLimiterError> {
        let permit = self
            .semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|_| RateLimiterError::AcquireFailed)?;

        let sem_clone = self.semaphore.clone();
        let period = self.refill_period;

        tokio::spawn(async move {
            sleep(period).await;
            sem_clone.add_permits(1);
        });

        Ok(RateLimiterPermit { _permit: permit })
    }
}

/// 用于保持 semaphore permit 生命周期
pub struct RateLimiterPermit {
    _permit: tokio::sync::OwnedSemaphorePermit,
}

/// ========================
/// Resilience Registries
/// ========================
#[derive(Debug)]
pub struct ResilienceRegistries {
    pub retry_configs: HashMap<String, Arc<RetryConfig>>,
    pub rate_limiters: HashMap<String, Arc<RateLimiter>>,
}

impl ResilienceRegistries {
    pub const DEFAULT_RETRY: &'static str = "global";
    pub const NON_IDEMPOTENT: &'static str = "non_idempotent";

    pub fn new() -> Self {
        let mut retry_configs = HashMap::new();
        retry_configs.insert(
            Self::DEFAULT_RETRY.into(),
            Arc::new(RetryConfig {
                max_attempts: 3,
                initial_delay: Duration::from_millis(50),
                multiplier: 2.0,
            }),
        );
        retry_configs.insert(
            Self::NON_IDEMPOTENT.into(),
            Arc::new(RetryConfig {
                max_attempts: 1,
                initial_delay: Duration::from_millis(50),
                multiplier: 1.0,
            }),
        );

        let mut rate_limiters = HashMap::new();
        rate_limiters.insert(
            "global".into(),
            Arc::new(RateLimiter::new(1200, Duration::from_secs(60))),
        );

        Self {
            retry_configs,
            rate_limiters,
        }
    }

    pub fn retry(&self, name: &str) -> Option<Arc<RetryConfig>> {
        self.retry_configs.get(name).cloned()
    }

    pub fn rate_limiter(&self, name: &str) -> Option<Arc<RateLimiter>> {
        self.rate_limiters.get(name).cloned()
    }
}

/// ========================
/// ResilientCall Builder
/// ========================
pub struct ResilientCall<T> {
    func: Box<
        dyn Fn() -> BoxFuture<'static, Result<T, Box<dyn std::error::Error + Send + Sync>>>
            + Send
            + Sync,
    >,
    retry_cfg: Option<Arc<RetryConfig>>,
    rate_limiter: Option<Arc<RateLimiter>>,
}

impl<T> ResilientCall<T>
where
    T: Send + 'static,
{
    pub fn new<F, Fut>(func: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<T, Box<dyn std::error::Error + Send + Sync>>> + Send + 'static,
    {
        Self {
            func: Box::new(move || func().boxed()),
            retry_cfg: None,
            rate_limiter: None,
        }
    }

    pub fn with_retry(mut self, retry_cfg: Arc<RetryConfig>) -> Self {
        self.retry_cfg = Some(retry_cfg);
        self
    }

    pub fn with_rate_limiter(mut self, limiter: Arc<RateLimiter>) -> Self {
        self.rate_limiter = Some(limiter);
        self
    }

    pub async fn call(&self) -> Result<T, Box<dyn std::error::Error + Send + Sync>> {
        let mut attempt = 0;

        loop {
            if let Some(ref limiter) = self.rate_limiter {
                let _permit = limiter.acquire().await;
            }

            let fut = (self.func)();
            let result = fut.await;

            if let Ok(val) = result {
                return Ok(val);
            }

            attempt += 1;
            if let Some(cfg) = &self.retry_cfg {
                if attempt >= cfg.max_attempts {
                    return result; // 超过最大尝试次数
                }
                let delay = cfg.delay_for_attempt(attempt);
                sleep(delay).await;
            } else {
                return result; // 没有 retry 配置直接返回
            }
        }
    }
}

/// ========================
/// 使用示例：结合 BinanceClient
/// ========================
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio;

    struct DummyClient;
    impl DummyClient {
        async fn ping(&self) -> Result<&'static str, Box<dyn std::error::Error + Send + Sync>> {
            Ok("pong")
        }
    }

    #[tokio::test]
    async fn test_resilient_call() {
        let registries = Arc::new(ResilienceRegistries::new());
        let client = DummyClient;

        let limiter = registries.get_rate_limiter("global").unwrap();
        let retry_cfg = registries.get_retry("global").unwrap();

        let result = ResilientCall::new(|| client.ping())
            .with_rate_limiter(limiter)
            .with_retry(retry_cfg)
            .call()
            .await
            .unwrap();

        assert_eq!(result, "pong");
    }
}
