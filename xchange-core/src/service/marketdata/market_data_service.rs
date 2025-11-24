use crate::dto::marketdata::funding_rate::FundingRate;
use crate::dto::marketdata::funding_rates::FundingRates;
use crate::dto::marketdata::order_book::OrderBook;
use crate::dto::marketdata::ticker::Ticker;
use crate::dto::marketdata::trades::Trades;
use crate::dto::meta::ExchangeHealth;
use crate::error::exchange_error::{ExchangeError, NotYetImplementedForExchangeError};
use crate::instrument::InstrumentDTO;
use crate::service::BaseService;
use crate::service::marketdata::params::Params;

/// Service to explore market data
pub trait MarketDataService: BaseService + Send + Sync {
    /// Get the exchange health
    fn exchange_health(&self) -> ExchangeHealth {
        ExchangeHealth::Online
    }

    /// Get ticker by instrument
    fn ticker(&self, _instrument: &InstrumentDTO, _args: &[&str]) -> Result<Ticker, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("get_ticker").into())
    }

    /// Get multiple tickers based on params
    fn tickers(&self, _params: &dyn Params) -> Result<Vec<Ticker>, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("get_tickers").into())
    }

    /// Get order book by instrument
    fn order_book(
        &self,
        _instrument: &InstrumentDTO,
        _args: &[&str],
    ) -> Result<OrderBook, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("get_order_book").into())
    }

    /// Get order book using params
    fn order_book_by_params(&self, _params: &dyn Params) -> Result<OrderBook, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("get_order_book").into())
    }

    /// Get trades by instrument
    fn trades(&self, _instrument: &InstrumentDTO, _args: &[&str]) -> Result<Trades, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("get_trades").into())
    }

    /// Get trades by params
    fn trades_by_params(&self, _params: &dyn Params) -> Result<Trades, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("get_trades").into())
    }

    /// Get candlestick data
    // fn get_candle_stick_data(
    //     &self,
    //     _currency_pair: &CurrencyPair,
    //     _params: &CandleStickDataParams,
    // ) -> Result<CandleStickData, ExchangeError> {
    //     Err(NotYetImplementedForExchangeError::with_message("get_candle_stick_data").into())
    // }

    /// Get all funding rates
    fn funding_rates(&self) -> Result<FundingRates, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("get_funding_rates").into())
    }

    /// Get funding rate for a specific instrument
    fn funding_rate(&self, _instrument: &InstrumentDTO) -> Result<FundingRate, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("get_funding_rate").into())
    }
}
