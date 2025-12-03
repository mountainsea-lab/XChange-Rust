use crate::currency::currency::Currency;
use crate::dto::account::account_info::AccountInfo;
use crate::dto::account::address_with_tag::AddressWithTag;
use crate::dto::account::fee::Fee;
use crate::dto::account::funding_record::FundingRecord;
use crate::error::exchange_error::{ExchangeError, NotYetImplementedForExchangeError};
use crate::instrument::InstrumentDTO;
use crate::service::BaseService;
use crate::service::account::params::RequestDepositAddressParams;
use crate::service::trade::params::TradeHistoryParams;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

/// AccountService inherits BaseService functionality.
#[async_trait]
pub trait AccountService: BaseService + Send + Sync {
    async fn account_info(&self) -> Result<AccountInfo, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("get_account_info").into())
    }

    // fn withdraw_funds_simple(
    //     &self,
    //     currency: Currency,
    //     amount: f64,
    //     address: impl Into<String>,
    // ) -> Result<String, ExchangeError> {
    //     let params = DefaultWithdrawFundsParams::new(currency, amount, address.into());
    //     self.withdraw_funds(&params)
    // }
    //
    // fn withdraw_funds(&self, params: &WithdrawFundsParams) -> Result<String, ExchangeError> {
    //     Err(NotYetImplementedForExchangeError::with_message(
    //         "withdraw_funds",
    //     )
    //         .into())
    // }

    async fn request_deposit_address(
        &self,
        _currency: Currency,
        _args: &[String],
    ) -> Result<String, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("request_deposit_address").into())
    }

    async fn request_deposit_address_with_params(
        &self,
        params: Arc<dyn RequestDepositAddressParams + Send + Sync>,
    ) -> Result<String, ExchangeError> {
        self.request_deposit_address(params.currency().clone(), params.extra_arguments())
            .await
    }

    async fn request_deposit_address_data(
        &self,
        currency: Currency,
        args: &[String],
    ) -> Result<AddressWithTag, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("request_deposit_address_data").into())
    }

    async fn request_deposit_address_data_with_params(
        &self,
        params: Arc<dyn RequestDepositAddressParams + Send + Sync>,
    ) -> Result<AddressWithTag, ExchangeError> {
        self.request_deposit_address_data(params.currency().clone(), params.extra_arguments())
            .await
    }

    async fn create_funding_history_params(
        &self,
    ) -> Result<Box<dyn TradeHistoryParams>, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("create_funding_history_params").into())
    }

    async fn get_funding_history(
        &self,
        params: Arc<dyn TradeHistoryParams + Send + Sync>,
    ) -> Result<Vec<FundingRecord>, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("get_funding_history").into())
    }

    async fn dynamic_trading_fees_by_instrument(
        &self,
    ) -> Result<HashMap<InstrumentDTO, Fee>, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message(
            "get_dynamic_trading_fees_by_instrument",
        )
        .into())
    }
}
