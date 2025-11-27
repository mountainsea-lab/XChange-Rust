use crate::dto::meta::binance_system::{BinanceSystemStatus, BinanceTime};
use crate::dto::meta::exchange_info::BinanceExchangeInfo;
use retrofit_rs::{RetrofitError, api, get};

/// Binance API Trait 抽象
#[api("https://api.binance.com")]
pub trait BinancePub {
    /// Fetch system status
    #[get("/sapi/v1/system/status")]
    async fn system_status(&self) -> Result<BinanceSystemStatus, RetrofitError>;

    /// Ping
    #[get("/api/v3/ping")]
    async fn ping(&self) -> Result<serde_json::Value, RetrofitError>;

    /// Get server time
    #[get("/api/v3/time")]
    async fn time(&self) -> Result<BinanceTime, RetrofitError>;

    /// Exchange info
    #[get("/api/v3/exchangeInfo")]
    async fn exchange_info(&self) -> Result<BinanceExchangeInfo, RetrofitError>;
    //
    // /// Depth / orderbook
    // async fn depth(
    //     &self,
    //     symbol: &str,
    //     limit: Option<u32>,
    // ) -> Result<BinanceOrderbook, RetrofitError>;
    //
    // /// Aggregate trades
    // async fn agg_trades(
    //     &self,
    //     symbol: &str,
    //     from_id: Option<u64>,
    //     start_time: Option<u64>,
    //     end_time: Option<u64>,
    //     limit: Option<u32>,
    // ) -> Result<Vec<BinanceAggTrades>, RetrofitError>;
    //
    // Klines / candlesticks
    // #[get("/api/v3/klines")]
    // async fn klines(
    //     &self,
    //     #[query] symbol: &str,
    //     #[query] interval: &str,
    //     #[query] limit: Option<u32>,
    //     #[query(name = "startTime")] start_time: Option<u64>,
    //     #[query(name = "endTime")] end_time: Option<u64>,
    // ) -> Result<Vec<Vec<serde_json::Value>>, RetrofitError>;
    //
    // /// 24h ticker for all symbols
    // async fn ticker_24h_all(&self) -> Result<Vec<BinanceTicker24h>, RetrofitError>;
    //
    // /// 24h ticker for a symbol
    // async fn ticker_24h(&self, symbol: &str) -> Result<BinanceTicker24h, RetrofitError>;
    //
    // /// Latest price for a symbol
    // async fn ticker_price(&self, symbol: &str) -> Result<BinancePrice, RetrofitError>;
    //
    // /// Latest price for all symbols
    // async fn ticker_all_prices(&self) -> Result<Vec<BinancePrice>, RetrofitError>;
    //
    // /// Best price/qty on the order book for all symbols
    // async fn ticker_all_book_tickers(&self) -> Result<Vec<BinancePriceQuantity>, RetrofitError>;
}
