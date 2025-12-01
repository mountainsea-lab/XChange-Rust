use crate::binance_exchange::BinanceExchange;
use crate::dto::BinanceError;
use crate::service::binance_account_service_raw::BinanceAccountServiceRaw;
use async_trait::async_trait;
use std::sync::Arc;
use xchange_core::dto::account::account_info::AccountInfo;
use xchange_core::error::exchange_error::{ExchangeError, NotYetImplementedForExchangeError};
use xchange_core::service::BaseService;

/// -------------------------
///  AccountService trait
/// -------------------------
#[async_trait]
pub trait AccountService: BaseService {
    /// 获取账户信息
    ///
    /// 默认实现：返回 NotImplemented 错误
    async fn get_account_info(&self) -> Result<AccountInfo, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("getAccountInfo".to_owned()).into())
    }
}

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
