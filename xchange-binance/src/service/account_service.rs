use async_trait::async_trait;
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
