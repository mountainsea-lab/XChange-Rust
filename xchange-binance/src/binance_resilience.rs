use std::sync::Arc;
use std::time::Duration;
use xchange_core::client::RateLimiter;
use xchange_core::client::ResilienceRegistries;

/// ========================
/// Binance 默认 Resilience 注册表
/// ========================
pub struct BinanceResilience {
    pub registries: ResilienceRegistries,
}

impl BinanceResilience {
    pub const REQUEST_WEIGHT: &'static str = "requestWeight";

    // Spot
    pub const ORDERS_PER_SECOND: &'static str = "ordersPerSecond";
    pub const RAW_REQUESTS: &'static str = "rawRequests";

    // Futures
    pub const ORDERS_PER_10_SECONDS: &'static str = "ordersPer10Seconds";
    pub const ORDERS_PER_MINUTE: &'static str = "ordersPerMinute";

    pub fn new_spot() -> Self {
        let mut registries = ResilienceRegistries::new();

        registries.rate_limiters.insert(
            Self::REQUEST_WEIGHT.into(),
            Arc::new(RateLimiter::new(6000, Duration::from_secs(60))),
        );
        registries.rate_limiters.insert(
            Self::ORDERS_PER_SECOND.into(),
            Arc::new(RateLimiter::new(10, Duration::from_secs(1))),
        );
        registries.rate_limiters.insert(
            Self::RAW_REQUESTS.into(),
            Arc::new(RateLimiter::new(61000, Duration::from_secs(5 * 60))),
        );

        Self { registries }
    }

    pub fn new_futures() -> Self {
        let mut registries = ResilienceRegistries::new();

        registries.rate_limiters.insert(
            Self::REQUEST_WEIGHT.into(),
            Arc::new(RateLimiter::new(2400, Duration::from_secs(60))),
        );
        registries.rate_limiters.insert(
            Self::ORDERS_PER_10_SECONDS.into(),
            Arc::new(RateLimiter::new(300, Duration::from_secs(10))),
        );
        registries.rate_limiters.insert(
            Self::ORDERS_PER_MINUTE.into(),
            Arc::new(RateLimiter::new(1200, Duration::from_secs(60))),
        );

        // Spot limiters unlimited for compatibility
        registries.rate_limiters.insert(
            Self::ORDERS_PER_SECOND.into(),
            Arc::new(RateLimiter::new(u32::MAX as usize, Duration::from_secs(1))),
        );
        registries.rate_limiters.insert(
            Self::RAW_REQUESTS.into(),
            Arc::new(RateLimiter::new(u32::MAX as usize, Duration::from_secs(1))),
        );

        Self { registries }
    }
}
