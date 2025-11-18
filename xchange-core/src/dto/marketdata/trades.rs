use crate::dto::marketdata::trade::Trade;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeSortType {
    SortByTimestamp,
    SortByID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trades {
    pub trades: Vec<Trade>,       // A vector to hold the collection of trades
    pub last_id: i64,             // Assuming last_id is a 64-bit integer (long in Java)
    pub next_page_cursor: String, // A string to hold the next page cursor
    pub trade_sort_type: TradeSortType, // The sorting type for trades
}
