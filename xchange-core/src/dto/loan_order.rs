use crate::dto::order::OrderType;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// A data structure representing an order for a loan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanOrder {
    /// Order type (e.g., Bid or Ask)
    pub order_type: OrderType,

    /// The loan currency (e.g., USD, EUR)
    pub currency: String,

    /// Amount to be ordered or the amount that was ordered
    pub original_amount: Decimal,

    /// Duration of the loan in days
    pub day_period: i32,

    /// An identifier that uniquely identifies the order
    pub id: String,

    /// The timestamp of the order according to the exchange's server, None if not provided
    pub timestamp: Option<DateTime<Utc>>,
}
