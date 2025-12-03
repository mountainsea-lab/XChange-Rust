use crate::binance_exchange::BinanceExchange;
use crate::dto::BinanceError;
use crate::dto::meta::binance_system::{BinanceSystemStatus, BinanceTime};
use crate::dto::meta::exchange_info::BinanceExchangeInfo;
use crate::service::market_data_service_inner::MarketDataInner;
use async_trait::async_trait;
use std::sync::Arc;
use xchange_core::dto::meta::ExchangeHealth;
use xchange_core::service::BaseService;
use xchange_core::service::marketdata::market_data_service::MarketDataService;

/// Binance Market Data Service
pub struct BinanceMarketDataService {
    inner: Arc<MarketDataInner>,
}

impl BinanceMarketDataService {
    /// 构造函数
    pub fn new(exchange: Arc<BinanceExchange>) -> Result<Self, BinanceError> {
        let client = MarketDataInner::new(exchange)?;
        Ok(Self {
            inner: Arc::new(client),
        })
    }
    // 调用宏批量代理方法
    delegate_client! {
        inner, {
            ping => (),
            binance_time => BinanceTime,
            system_status => BinanceSystemStatus,
            exchange_info => BinanceExchangeInfo,
            future_exchange_info => BinanceExchangeInfo,
        }
    }
}

/// 实现 MarketDataService trait
#[async_trait]
impl MarketDataService for BinanceMarketDataService {
    /// 默认实现 ExchangeHealth
    async fn exchange_health(&self) -> ExchangeHealth {
        // 调用 system_status()，由宏生成的 async 方法
        match self.system_status().await {
            Ok(status) => {
                // Binance 返回 0 = 系统正常
                if status.status == 0 {
                    ExchangeHealth::Online
                } else {
                    ExchangeHealth::Offline
                }
            }
            Err(_) => ExchangeHealth::Offline, // 调用失败 → OFFLINE
        }
    }

    // 其他 trait 方法可以继续占位实现
}
impl BaseService for BinanceMarketDataService {}
