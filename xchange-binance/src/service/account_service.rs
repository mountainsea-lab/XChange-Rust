use crate::binance_exchange::BinanceExchange;
use crate::dto::BinanceError;
use crate::service::binance_account_service_raw::BinanceAccountServiceRaw;
use async_trait::async_trait;
use std::any::Any;
use std::sync::Arc;
use xchange_core::dto::account::account_info::AccountInfo;
use xchange_core::error::exchange_error::ExchangeError;
use xchange_core::service::BaseService;
use xchange_core::service::account::account_service::AccountService;

#[derive(Clone)]
pub struct BinanceAccountService {
    raw: Arc<BinanceAccountServiceRaw>,
}

impl BinanceAccountService {
    pub fn new(exchange: Arc<BinanceExchange>) -> Result<Self, BinanceError> {
        Ok(Self {
            raw: Arc::new(BinanceAccountServiceRaw::new(exchange)?),
        })
    }
}

#[async_trait]
impl AccountService for BinanceAccountService {
    async fn account_info(&self) -> Result<AccountInfo, ExchangeError> {
        // future implementation:
        // - 根据 ExchangeType 调用 raw.account / raw.futures_account
        // - 使用 BinanceAdapters 转换模型
        // - 捕获 BinanceError → 转换为 ExchangeError

        unimplemented!("BinanceAccountService::get_account_info is not implemented yet");
    }
}
impl BaseService for BinanceAccountService {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
