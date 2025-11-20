use crate::currency::currency::Currency;
use crate::dto::meta::currency_metadata::CurrencyMetaData;
use crate::dto::meta::instrument_metadata::InstrumentMetaData;
use crate::dto::meta::rate_limit::RateLimit;
use crate::instrument::InstrumentDTO;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Exchange metadata containing instruments, currencies and rate limits.
///
/// This is loaded at startup and merges local JSON metadata + online exchange info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeMetaData {
    /// Map of InstrumentDTO -> InstrumentMetaData
    #[serde(rename = "currency_pairs")]
    pub instruments: HashMap<InstrumentDTO, InstrumentMetaData>,

    /// Map of Currency -> CurrencyMetaData
    #[serde(rename = "currencies")]
    pub currencies: HashMap<Currency, CurrencyMetaData>,

    /// Public API rate limits
    #[serde(rename = "public_rate_limits", default)]
    pub public_rate_limits: Vec<RateLimit>,

    /// Private API rate limits
    #[serde(rename = "private_rate_limits", default)]
    pub private_rate_limits: Vec<RateLimit>,

    /// If true, public & private calls share the private rate limits
    #[serde(rename = "share_rate_limits", default)]
    pub share_rate_limits: bool,
}

impl ExchangeMetaData {
    /// Static helper, same semantics as Java:
    ///
    /// Return **max poll delay in millis**, or None if no rate limits.
    pub fn get_poll_delay_millis(rate_limits: &[RateLimit]) -> Option<u64> {
        rate_limits.iter().map(RateLimit::poll_delay_millis).max()
    }

    /// Serialize into compact JSON (equivalent to Java toJSONString)
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).expect("Serialize ExchangeMetaData")
    }
}
