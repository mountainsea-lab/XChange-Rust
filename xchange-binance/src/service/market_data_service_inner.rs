use crate::binance::BinanceAdapters;
use crate::binance_exchange::BinanceExchange;
use crate::binance_resilience::REQUEST_WEIGHT_RATE_LIMITER;
use crate::client::binance_futures::BinanceFuturesAuthed;
use crate::client::binance_spot::BinanceAuthed;
use crate::dto::BinanceError;
use crate::dto::marketdata::KlineInterval;
use crate::dto::marketdata::binance_kline::BinanceKline;
use crate::dto::meta::binance_system::{BinanceSystemStatus, BinanceTime};
use crate::dto::meta::exchange_info::BinanceExchangeInfo;
use crate::service::binance_base_service::BinanceBaseService;
use retrofit_rs::Query;
use std::sync::Arc;
use xchange_core::client::{ResilientCall, boxed};
use xchange_core::currency::currency_pair::CurrencyPair;
use xchange_core::instrument::InstrumentKind;

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

        let auth_client = self.base.client.spot.clone();

        let mut resilient = ResilientCall::new(move || {
            let auth_client = auth_client.clone();
            async move { auth_client.ping().await.map_err(boxed) }
        });

        if let Some(limiter) = limit {
            resilient = resilient.with_rate_limiter(limiter);
        }

        resilient.call().await?;

        Ok(())
    }

    pub async fn system_status(&self) -> Result<BinanceSystemStatus, BinanceError> {
        self.base.system_status().await
    }

    pub async fn binance_time(&self) -> Result<BinanceTime, BinanceError> {
        let retry = self
            .base
            .exchange
            .resilience_registries
            .retry(REQUEST_WEIGHT_RATE_LIMITER);
        let limit = self
            .base
            .exchange
            .resilience_registries
            .rate_limiter(REQUEST_WEIGHT_RATE_LIMITER)
            .as_ref()
            .cloned();

        let auth_client = self.base.client.spot.clone();

        let mut resilient = ResilientCall::new(move || {
            let auth_client = auth_client.clone();
            async move { auth_client.time().await.map_err(boxed) }
        });

        if let Some(retry) = retry {
            resilient = resilient.with_retry(retry)
        }
        if let Some(limiter) = limit {
            resilient = resilient.with_rate_limiter(limiter);
        }

        resilient.call().await.map_err(|e| BinanceError::from(e))
    }

    pub async fn exchange_info(&self) -> Result<BinanceExchangeInfo, BinanceError> {
        let retry = self
            .base
            .exchange
            .resilience_registries
            .retry(REQUEST_WEIGHT_RATE_LIMITER);
        let limit = self
            .base
            .exchange
            .resilience_registries
            .rate_limiter(REQUEST_WEIGHT_RATE_LIMITER)
            .as_ref()
            .cloned();

        let auth_client = self.base.client.spot.clone();

        let mut resilient = ResilientCall::new(move || {
            let auth_client = auth_client.clone();
            async move { auth_client.exchange_info().await.map_err(boxed) }
        });

        if let Some(retry) = retry {
            resilient = resilient.with_retry(retry)
        }
        if let Some(limiter) = limit {
            resilient = resilient.with_rate_limiter(limiter);
        }

        resilient.call().await.map_err(|e| BinanceError::from(e))
    }

    pub async fn future_exchange_info(&self) -> Result<BinanceExchangeInfo, BinanceError> {
        let retry = self
            .base
            .exchange
            .resilience_registries
            .retry(REQUEST_WEIGHT_RATE_LIMITER);
        let limit = self
            .base
            .exchange
            .resilience_registries
            .rate_limiter(REQUEST_WEIGHT_RATE_LIMITER)
            .as_ref()
            .cloned();

        let auth_client =
            self.base.client.futures.clone().ok_or_else(|| {
                boxed(BinanceError::ClientNotInitialized("futures client".into()))
            })?;

        let mut resilient = ResilientCall::new(move || {
            let auth_client = auth_client.clone();
            async move { auth_client.exchange_info().await.map_err(boxed) }
        });

        if let Some(retry) = retry {
            resilient = resilient.with_retry(retry)
        }
        if let Some(limiter) = limit {
            resilient = resilient.with_rate_limiter(limiter);
        }

        resilient.call().await.map_err(|e| BinanceError::from(e))
    }

    pub async fn last_kline(
        &self,
        pair: CurrencyPair,
        interval: KlineInterval,
    ) -> Result<BinanceKline, BinanceError> {
        let klines = self.klines(pair, interval, Some(1), None, None).await?;

        klines
            .into_iter()
            .next()
            .ok_or_else(|| BinanceError::Message("No kline returned".into()))
    }

    pub async fn klines_default_limit(
        &self,
        pair: CurrencyPair,
        interval: KlineInterval,
    ) -> Result<Vec<BinanceKline>, BinanceError> {
        self.klines(pair, interval, None, None, None).await
    }

    pub async fn klines(
        &self,
        pair: CurrencyPair,
        interval: KlineInterval,
        limit: Option<u32>,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Result<Vec<BinanceKline>, BinanceError> {
        // Resilience 配置
        let retry = self
            .base
            .exchange
            .resilience_registries
            .retry(REQUEST_WEIGHT_RATE_LIMITER);
        let limiter = self
            .base
            .exchange
            .resilience_registries
            .rate_limiter(REQUEST_WEIGHT_RATE_LIMITER)
            .as_ref()
            .cloned();

        // 提前准备常量数据
        let spot_client = self.base.client.spot.clone();
        let instrument_kind = InstrumentKind::CurrencyPair(pair.clone());
        let pair_symbol = BinanceAdapters::to_symbol(&instrument_kind);
        let interval_code = interval.code().to_string();

        // Query 转换提前构造
        let limit_q: Option<Query<u16>> = limit.map(|v| Query(v as u16));
        let start_q: Option<Query<u64>> = start_time.map(Query);
        let end_q: Option<Query<u64>> = end_time.map(Query);

        // ResilientCall
        let mut resilient = ResilientCall::new({
            // 全部 clone，闭包里直接 move
            let spot_client = spot_client.clone();
            let instrument_kind = instrument_kind.clone();
            let pair_symbol = pair_symbol.clone();
            let interval = interval.clone();
            let interval_code = interval_code.clone();
            let limit_q = limit_q.clone();
            let start_q = start_q.clone();
            let end_q = end_q.clone();

            move || {
                let spot_client = spot_client.clone();
                let instrument_kind = instrument_kind.clone();
                let interval = interval.clone();
                let pair_symbol = pair_symbol.clone();
                let interval_code = interval_code.clone();
                let limit_q = limit_q.clone();
                let start_q = start_q.clone();
                let end_q = end_q.clone();

                async move {
                    let raw: Vec<Vec<serde_json::Value>> = spot_client
                        .klines(
                            Query(pair_symbol.as_str()),
                            Query(interval_code.as_str()),
                            limit_q,
                            start_q,
                            end_q,
                        )
                        .await
                        .map_err(boxed)?;

                    Ok(raw
                        .into_iter()
                        .map(|v| BinanceKline::new(&instrument_kind, &interval, v.as_slice()))
                        .collect::<Vec<BinanceKline>>())
                }
            }
        });

        // 应用 retry / rate limiter
        if let Some(r) = retry {
            resilient = resilient.with_retry(r);
        }
        if let Some(l) = limiter {
            resilient = resilient.with_rate_limiter(l);
        }

        resilient.call().await.map_err(|e| BinanceError::from(e))
    }

    pub async fn future_last_kline(
        &self,
        pair: CurrencyPair,
        interval: KlineInterval,
    ) -> Result<BinanceKline, BinanceError> {
        let future_klines = self
            .future_klines(pair, interval, Some(1), None, None)
            .await?;

        future_klines
            .into_iter()
            .next()
            .ok_or_else(|| BinanceError::Message("No kline returned".into()))
    }

    pub async fn future_klines_default_limit(
        &self,
        pair: CurrencyPair,
        interval: KlineInterval,
    ) -> Result<Vec<BinanceKline>, BinanceError> {
        self.future_klines(pair, interval, None, None, None).await
    }

    pub async fn future_klines(
        &self,
        pair: CurrencyPair,
        interval: KlineInterval,
        limit: Option<u32>,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Result<Vec<BinanceKline>, BinanceError> {
        // Resilience 配置
        let retry = self
            .base
            .exchange
            .resilience_registries
            .retry(REQUEST_WEIGHT_RATE_LIMITER);
        let limiter = self
            .base
            .exchange
            .resilience_registries
            .rate_limiter(REQUEST_WEIGHT_RATE_LIMITER)
            .as_ref()
            .cloned();

        // 提前准备常量数据
        let future_client =
            self.base.client.futures.clone().ok_or_else(|| {
                boxed(BinanceError::ClientNotInitialized("futures client".into()))
            })?;

        let instrument_kind = InstrumentKind::CurrencyPair(pair.clone());
        let pair_symbol = BinanceAdapters::to_symbol(&instrument_kind);
        let interval_code = interval.code().to_string();

        // Query 转换提前构造
        let limit_q: Option<Query<u16>> = limit.map(|v| Query(v as u16));
        let start_q: Option<Query<u64>> = start_time.map(Query);
        let end_q: Option<Query<u64>> = end_time.map(Query);

        // ResilientCall
        let mut resilient = ResilientCall::new({
            // 全部 clone，闭包里直接 move
            let future_client = future_client.clone();
            let instrument_kind = instrument_kind.clone();
            let pair_symbol = pair_symbol.clone();
            let interval = interval.clone();
            let interval_code = interval_code.clone();
            let limit_q = limit_q.clone();
            let start_q = start_q.clone();
            let end_q = end_q.clone();

            move || {
                let future_client = future_client.clone();
                let instrument_kind = instrument_kind.clone();
                let interval = interval.clone();
                let pair_symbol = pair_symbol.clone();
                let interval_code = interval_code.clone();
                let limit_q = limit_q.clone();
                let start_q = start_q.clone();
                let end_q = end_q.clone();

                async move {
                    let raw: Vec<Vec<serde_json::Value>> = future_client
                        .klines(
                            Query(pair_symbol.as_str()),
                            Query(interval_code.as_str()),
                            limit_q,
                            start_q,
                            end_q,
                        )
                        .await
                        .map_err(boxed)?;

                    Ok(raw
                        .into_iter()
                        .map(|v| BinanceKline::new(&instrument_kind, &interval, v.as_slice()))
                        .collect::<Vec<BinanceKline>>())
                }
            }
        });

        // 应用 retry / rate limiter
        if let Some(r) = retry {
            resilient = resilient.with_retry(r);
        }
        if let Some(l) = limiter {
            resilient = resilient.with_rate_limiter(l);
        }

        resilient.call().await.map_err(|e| BinanceError::from(e))
    }
}
