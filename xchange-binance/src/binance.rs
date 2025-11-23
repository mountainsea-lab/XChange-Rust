use async_trait::async_trait;
use crate::dto::BinanceError;
use crate::dto::meta::binance_system::{BinanceSystemStatus, BinanceTime};
use crate::dto::meta::exchange_info::BinanceExchangeInfo;

/// Binance API Trait 抽象
#[async_trait]
pub trait Binance {
    /// Fetch system status
    async fn system_status(&self) -> Result<BinanceSystemStatus, BinanceError>;

    /// Ping
    async fn ping(&self) -> Result<serde_json::Value, BinanceError>;

    /// Get server time
    async fn time(&self) -> Result<BinanceTime, BinanceError>;

    /// Exchange info
    async fn exchange_info(&self) -> Result<BinanceExchangeInfo, BinanceError>;
    //
    // /// Depth / orderbook
    // async fn depth(
    //     &self,
    //     symbol: &str,
    //     limit: Option<u32>,
    // ) -> Result<BinanceOrderbook, BinanceError>;
    //
    // /// Aggregate trades
    // async fn agg_trades(
    //     &self,
    //     symbol: &str,
    //     from_id: Option<u64>,
    //     start_time: Option<u64>,
    //     end_time: Option<u64>,
    //     limit: Option<u32>,
    // ) -> Result<Vec<BinanceAggTrades>, BinanceError>;
    //
    // /// Klines / candlesticks
    // async fn klines(
    //     &self,
    //     symbol: &str,
    //     interval: &str,
    //     limit: Option<u32>,
    //     start_time: Option<u64>,
    //     end_time: Option<u64>,
    // ) -> Result<Vec<Vec<serde_json::Value>>, BinanceError>;
    //
    // /// 24h ticker for all symbols
    // async fn ticker_24h_all(&self) -> Result<Vec<BinanceTicker24h>, BinanceError>;
    //
    // /// 24h ticker for a symbol
    // async fn ticker_24h(&self, symbol: &str) -> Result<BinanceTicker24h, BinanceError>;
    //
    // /// Latest price for a symbol
    // async fn ticker_price(&self, symbol: &str) -> Result<BinancePrice, BinanceError>;
    //
    // /// Latest price for all symbols
    // async fn ticker_all_prices(&self) -> Result<Vec<BinancePrice>, BinanceError>;
    //
    // /// Best price/qty on the order book for all symbols
    // async fn ticker_all_book_tickers(&self) -> Result<Vec<BinancePriceQuantity>, BinanceError>;
}