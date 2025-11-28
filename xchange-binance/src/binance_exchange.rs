use crate::binance_time_provider::BinanceTimeProvider;
use std::sync::Arc;
use xchange_core::client::ResilienceRegistries;
use xchange_core::exchange::BaseExchange;

pub struct BinanceExchange {
    pub base_exchange: BaseExchange,
    pub resilience_registries: ResilienceRegistries,
    pub timestamp_factory: Arc<BinanceTimeProvider>,
}
