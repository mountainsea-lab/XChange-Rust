use crate::dto::BinanceError;
use crate::dto::meta::binance_system::BinanceTime;
use futures::future::FutureExt;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tracing::trace;
use xchange_core::ValueFactory;
use xchange_core::client::{ResilienceRegistries, ResilientCall};
use xchange_core::exchange_specification::ResilienceSpecification;

#[derive(Debug)]
pub struct BinanceTimeProvider {
    resilience_specification: ResilienceSpecification,
    registries: Arc<ResilienceRegistries>,
    delta_server_time: Option<i64>,
    delta_server_time_expire: Option<SystemTime>,
}

impl BinanceTimeProvider {
    pub fn new(
        resilience_specification: ResilienceSpecification,
        registries: Arc<ResilienceRegistries>,
    ) -> Self {
        Self {
            resilience_specification,
            registries,
            delta_server_time: None,
            delta_server_time_expire: None,
        }
    }

    /// 对应 Java createValue()，返回当前系统时间毫秒
    pub fn create_value(&self) -> Result<i64, BinanceError> {
        let duration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| BinanceError::TimeProvider(Box::new(e)))?;
        Ok(duration.as_millis() as i64)
    }

    /// 清空 delta，强制下一次刷新
    pub fn clear_delta_server_time(&mut self) {
        self.delta_server_time = None;
        self.delta_server_time_expire = None;
    }

    /// 异步获取服务器时间并缓存 delta
    pub async fn delta_server_time<F, Fut>(&mut self, fetch: F) -> Result<i64, BinanceError>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<BinanceTime, BinanceError>> + Send + 'static,
    {
        let now = SystemTime::now();
        if let (Some(delta), Some(expire)) = (self.delta_server_time, self.delta_server_time_expire)
        {
            if now < expire {
                return Ok(delta
                    + now
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as i64);
            }
        }

        let binance_time = self.binance_time(fetch).await?;
        let server_time_millis = binance_time.server_time;

        let system_millis = now
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        // 缓存 10 分钟
        self.delta_server_time = Some(server_time_millis - system_millis);
        self.delta_server_time_expire = Some(now + Duration::from_secs(600));

        // trace log
        trace!(
            "delta_server_time updated: server={} system={} delta={}",
            server_time_millis,
            system_millis,
            self.delta_server_time.unwrap()
        );

        Ok(server_time_millis)
    }

    /// 核心 Resilience 调用（限流 + 重试）
    async fn binance_time<F, Fut>(&self, fetch: F) -> Result<BinanceTime, BinanceError>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<BinanceTime, BinanceError>> + Send + 'static,
    {
        // 使用 Arc 包装 fetch，避免 move 错误
        let fetch = Arc::new(fetch);

        let retry_cfg = self
            .registries
            .retry("time")
            .ok_or(BinanceError::RetryConfigNotFound)?;
        let limiter = self.registries.rate_limiter("REQUEST_WEIGHT");

        let mut call: ResilientCall<BinanceTime> = ResilientCall::new({
            let fetch = fetch.clone();
            move || {
                let fetch = fetch.clone();
                async move {
                    fetch()
                        .await
                        .map_err(|e| Box::<dyn std::error::Error + Send + Sync>::from(e))
                }
                .boxed()
            }
        })
        .with_retry(retry_cfg);

        if let Some(limiter) = limiter {
            call = call.with_rate_limiter(limiter);
        }

        call.call().await.map_err(|e| BinanceError::TimeProvider(e))
    }
}

impl ValueFactory<u64> for BinanceTimeProvider {
    fn create(&self) -> u64 {
        self.create()
    }
}
