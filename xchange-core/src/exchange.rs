use crate::client::resilience_registries::ResilienceRegistries;
use crate::dto::meta::exchange_metadata::ExchangeMetaData;
use crate::error::exchange_error::{ExchangeError, NotYetImplementedForExchangeError};
use crate::exchange_specification::ExchangeSpecification;
use crate::instrument::Instrument;
use crate::service::account::account_service::AccountService;
use crate::service::marketdata::market_data_service::MarketDataService;
use crate::service::trade::trade_service::TradeService;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Exchange: Send + Sync {
    const USE_SANDBOX: &'static str = "Use_Sandbox";

    fn exchange_specification(&self) -> &ExchangeSpecification;

    fn exchange_meta_data(&self) -> &ExchangeMetaData;

    fn exchange_instruments(&self) -> Vec<Arc<dyn Instrument>>;

    fn default_exchange_specification(&self) -> ExchangeSpecification;

    fn apply_specification(&mut self, spec: ExchangeSpecification);

    fn get_market_data_service(&self) -> Arc<dyn MarketDataService>;

    fn get_trade_service(&self) -> Arc<dyn TradeService>;

    fn get_account_service(&self) -> Arc<dyn AccountService>;

    async fn remote_init(&mut self) -> Result<(), ExchangeError> {
        Ok(())
    }

    async fn get_resilience_registries(&self) -> Result<Arc<ResilienceRegistries>, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message(
            "Resilience features not implemented".to_string(),
        )
        .into())
    }
}
