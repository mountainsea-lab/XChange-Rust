use xchange_core::currency::currency_pair::CurrencyPair;
use xchange_core::dto::marketdata::ticker::Ticker;
use xchange_core::dto::meta::ExchangeHealth;
use xchange_core::error::exchange_error::{ExchangeError, NotYetImplementedForExchangeError};
use xchange_core::service::BaseService;

/// Trait representing market data services for an Exchange
pub trait MarketDataService: BaseService {
    /// Returns the current exchange health
    /// Default implementation returns `ExchangeHealth::Online`
    fn get_exchange_health(&self) -> ExchangeHealth {
        ExchangeHealth::Online
    }

    /// Get a ticker representing the current exchange rate for a currency pair
    ///
    /// Default implementation returns an error indicating not yet implemented
    fn get_ticker(
        &self,
        _currency_pair: &CurrencyPair,
        _args: Option<&[&str]>,
    ) -> Result<Ticker, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("getAccountInfo".to_owned()).into())
    }
}
