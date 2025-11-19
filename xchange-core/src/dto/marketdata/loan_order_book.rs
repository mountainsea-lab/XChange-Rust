use crate::dto::trade::fixed_rate_loan_order::FixedRateLoanOrder;
use crate::dto::trade::floating_rate_loan_order::FloatingRateLoanOrder;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// DTO representing the exchange loan order book
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoanOrderBook {
    pub fixed_rate_asks: Vec<FixedRateLoanOrder>,
    pub fixed_rate_bids: Vec<FixedRateLoanOrder>,
    pub floating_rate_asks: Vec<FloatingRateLoanOrder>,
    pub floating_rate_bids: Vec<FloatingRateLoanOrder>,
    pub timestamp: DateTime<Utc>,
}
