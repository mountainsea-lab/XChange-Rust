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
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub limit: i32,
    pub interval: String,
    pub interval_num: i32,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub status: String,
    pub base_asset: String,
    pub quote_asset: String,

    pub base_asset_precision: u8,          // 数字
    pub quote_asset_precision: Option<u8>, // 数字
    pub quote_precision: u8,               // 数字

    pub iceberg_allowed: Option<bool>,
    pub oco_allowed: Option<bool>,
    pub is_margin_trading_allowed: Option<bool>,
    pub is_spot_trading_allowed: Option<bool>,

    pub contract_type: Option<String>,
    pub delivery_date: Option<i64>,
    pub onboard_date: Option<i64>,

    pub order_types: Vec<String>,
    pub filters: Vec<Filter>,
    pub permissions: Option<Vec<String>>,
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinanceExchangeInfo {
    pub timezone: String,

    pub symbols: Vec<Symbol>,

    #[serde(deserialize_with = "xchange_core::utils::deserialize_timestamp")]
    pub server_time: DateTime<Utc>,

    pub rate_limits: Vec<RateLimit>,

    pub exchange_filters: Vec<String>,

    pub permissions: Option<Vec<String>>,
}
