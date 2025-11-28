pub mod meta;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;
use xchange_core::rescu::params_digest::DigestError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExchangeType {
    Spot,
    Futures,
    Inverse,
    PortfolioMargin,
}

impl fmt::Display for ExchangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExchangeType::Spot => "SPOT",
            ExchangeType::Futures => "FUTURES",
            ExchangeType::Inverse => "INVERSE",
            ExchangeType::PortfolioMargin => "PORTFOLIO_MARGIN",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Error)]
pub enum BinanceError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Binance returned error: {0}")]
    Binance(#[from] BinanceException),

    #[error("Unexpected HTTP status {0}: {1}")]
    HttpStatus(reqwest::StatusCode, String),

    #[error("Retrofit error: {0}")]
    Retrofit(#[from] retrofit_rs::RetrofitError),

    #[error("Time provider error: {0}")]
    TimeProvider(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("Retry config not found")]
    RetryConfigNotFound,

    #[error("Acquire rate limiter failed: {0}")]
    AcquireRateLimiter(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("Digest error: {0}")]
    Digest(#[from] DigestError),

    #[error("Service Not Initialized: {0}")]
    ServiceNotInitialized(String),
}

/// Binance API 返回的业务错误，例如签名错误、参数错误、权限不足等。
///
/// Java 中的 BinanceException 拥有：
/// - code（业务错误码）
/// - msg（错误信息）
/// - headers（HTTP 响应头）
///
/// Rust 版本保持一致的结构。
#[derive(Debug, Error, Clone, Deserialize)]
pub struct BinanceException {
    /// 业务错误码，如 -1000, -1021, -2015 等
    pub code: i32,

    /// 返回的错误信息（Java: msg）
    #[serde(rename = "msg")]
    pub msg: String,

    /// HTTP 响应头（可选，因为非所有错误都有）
    #[serde(skip)]
    pub headers: Option<HashMap<String, Vec<String>>>,
}

impl fmt::Display for BinanceException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Binance error {}: {}", self.code, self.msg)
    }
}

impl BinanceException {
    pub fn new(code: i32, msg: impl Into<String>) -> Self {
        Self {
            code,
            msg: msg.into(),
            headers: None,
        }
    }

    /// 设置 HTTP 响应头（对应 Java 的 setResponseHeaders）
    pub fn set_headers(&mut self, headers: HashMap<String, Vec<String>>) {
        self.headers = Some(headers);
    }
}
