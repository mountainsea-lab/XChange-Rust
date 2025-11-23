use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    #[serde(rename = "maxPrice")]
    pub max_price: Option<String>,

    #[serde(rename = "filterType")]
    pub filter_type: Option<String>,

    #[serde(rename = "tickSize")]
    pub tick_size: Option<String>,

    #[serde(rename = "minPrice")]
    pub min_price: Option<String>,

    #[serde(rename = "minQty")]
    pub min_qty: Option<String>,

    #[serde(rename = "maxQty")]
    pub max_qty: Option<String>,

    #[serde(rename = "stepSize")]
    pub step_size: Option<String>,

    #[serde(rename = "minNotional")]
    pub min_notional: Option<String>,

    #[serde(rename = "maxNotional")]
    pub max_notional: Option<String>,

    #[serde(rename = "notional")]
    pub notional: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub limit: String,
    pub interval: String,
    pub interval_num: String,
    pub rate_limit_type: String,
}

impl std::fmt::Display for RateLimit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RateLimit [limit = {}, interval = {}, intervalNum = {}, rateLimitType = {}]",
            self.limit, self.interval, self.interval_num, self.rate_limit_type
        )
    }
}

/// Rust equivalent of Java `Symbol`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub quote_asset: String,
    pub iceberg_allowed: String,
    pub oco_allowed: String,
    pub is_margin_trading_allowed: String,
    pub is_spot_trading_allowed: String,
    pub base_asset: String,
    pub symbol: String,
    pub status: String,
    pub contract_type: String,
    pub quote_precision: String,
    pub quote_asset_precision: String,
    pub base_asset_precision: String,
    pub delivery_date: Option<i64>,
    pub onboard_date: Option<i64>,
    pub order_types: Vec<String>,
    pub filters: Vec<Filter>,
    pub permissions: Vec<String>,
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceExchangeInfo {
    pub timezone: String,

    pub symbols: Vec<Symbol>,

    #[serde(deserialize_with = "xchange_core::utils::deserialize_timestamp")]
    pub server_time: DateTime<Utc>,

    pub rate_limits: Vec<RateLimit>,

    pub exchange_filters: Vec<String>,

    pub permissions: Vec<String>,
}
