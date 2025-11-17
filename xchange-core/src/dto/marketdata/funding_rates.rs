use crate::dto::marketdata::funding_rate::FundingRate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingRates {
    #[serde(rename = "fundingRates")]
    pub funding_rates: Vec<FundingRate>,
}

impl FundingRates {
    pub fn new(funding_rates: Vec<FundingRate>) -> Self {
        Self { funding_rates }
    }
}
