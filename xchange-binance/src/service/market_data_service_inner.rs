use crate::binance_exchange::BinanceExchange;
use crate::binance_resilience::REQUEST_WEIGHT_RATE_LIMITER;
use crate::client::binance_authed::BinanceAuthed;
use crate::dto::BinanceError;
use crate::dto::meta::binance_system::BinanceTime;
use crate::dto::meta::exchange_info::BinanceExchangeInfo;
use crate::service::binance_base_service::BinanceBaseService;
use std::sync::Arc;
use xchange_core::client::{ResilientCall, boxed};

/// 公共封装层：Binance Market Data 客户端
pub struct MarketDataInner {
    base: Arc<BinanceBaseService>,
}
impl MarketDataInner {
    pub fn new(exchange: Arc<BinanceExchange>) -> Result<Self, BinanceError> {
        let base = BinanceBaseService::new(exchange.clone())
            .map_err(|e| BinanceError::ServiceNotInitialized(e.to_string()))?;

        Ok(Self {
            base: Arc::new(base),
        })
    }

    // todo 通用模板，把这种 auth client + resilient call + rate limiter + retry 封装成 可复用函数/宏，让所有 API 调用都可以一行写完
    pub async fn ping(&self) -> Result<(), BinanceError> {
        let limit = self
            .base
            .exchange
            .resilience_registries
            .rate_limiter(REQUEST_WEIGHT_RATE_LIMITER)
            .as_ref()
            .cloned();

        let auth_client = self.base.client.auth.clone();

        let mut resilient = ResilientCall::new(move || {
            let auth_client = auth_client.clone();
            async move {
                auth_client
                    .ok_or_else(|| {
                        BinanceError::ClientNotInitialized("auth client not initialized".into())
                    })?
                    .ping()
                    .await
                    .map_err(boxed)
            }
        });

        if let Some(limiter) = limit {
            resilient = resilient.with_rate_limiter(limiter);
        }

        resilient.call().await?;

        Ok(())
    }
    pub async fn binance_time(&self) -> Result<BinanceTime, BinanceError> {
        // let resilience_registries = self.base.exchange.resilience_registries;
        // let retry = resilience_registries.retry(REQUEST_WEIGHT_RATE_LIMITER);
        // let limit = resilience_registries.rate_limiter(REQUEST_WEIGHT_RATE_LIMITER);
        // ResilientCall::new(|| async {
        //     self.base.client..binance_time().await.map_err(boxed)
        // })
        //     .with_rate_limiter(limit)
        //     .call()
        //     .await?;
        // TODO: 调用 exchange API，并加 retry / rate limiter
        unimplemented!("get_exchange_info not implemented yet")
    }

    pub async fn exchange_info(&self) -> Result<BinanceExchangeInfo, BinanceError> {
        // TODO: 调用 exchange API，并加 retry / rate limiter
        unimplemented!("get_exchange_info not implemented yet")
    }

    pub async fn future_exchange_info(&self) -> Result<BinanceExchangeInfo, BinanceError> {
        // TODO: 调用 exchange API，并加 retry / rate limiter
        unimplemented!("get_exchange_info not implemented yet")
    }
}
