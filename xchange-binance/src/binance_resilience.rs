use std::time::Duration;
use xchange_core::client::RateLimiter;
use xchange_core::client::resilience_registries::ResilienceRegistries;

pub struct BinanceResilience;

impl BinanceResilience {
    pub const REQUEST_WEIGHT_RATE_LIMITER: &'static str = "requestWeight";
    pub const ORDERS_PER_SECOND_RATE_LIMITER: &'static str = "ordersPerSecond";
    pub const ORDERS_PER_DAY_RATE_LIMITER: &'static str = "ordersPerDay";

    pub fn create_registries() -> ResilienceRegistries {
        // 先创建默认的 registries（带默认 retry & rateLimiters）
        let mut registries = ResilienceRegistries::new();

        // --- requestWeight (1200 / 1min) ---
        registries.rate_limiters.insert(
            Self::REQUEST_WEIGHT_RATE_LIMITER.into(),
            RateLimiter::new(1200, Duration::from_secs(60)),
        );

        // --- ordersPerSecond (10 / 1s) ---
        registries.rate_limiters.insert(
            Self::ORDERS_PER_SECOND_RATE_LIMITER.into(),
            RateLimiter::new(10, Duration::from_secs(1)),
        );

        // --- ordersPerDay (200000 / 1 day) ---
        registries.rate_limiters.insert(
            Self::ORDERS_PER_DAY_RATE_LIMITER.into(),
            RateLimiter::new(200_000, Duration::from_secs(86_400)),
        );

        registries
    }
}
