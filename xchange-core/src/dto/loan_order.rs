use crate::dto::order::OrderType;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

/// A data structure representing an order for a loan.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
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

impl LoanOrder {
    /// Constructor for creating a new LoanOrder.
    ///
    /// # Parameters
    /// - `order_type`: The type of order (Bid or Ask).
    /// - `currency`: The currency of the loan.
    /// - `original_amount`: The amount to be ordered or that was ordered.
    /// - `day_period`: Duration of the loan in days.
    /// - `id`: A unique identifier for the order.
    /// - `timestamp`: The timestamp of the order, `None` if not provided.
    ///
    /// # Returns
    /// A new instance of `LoanOrder`.
    pub fn new(
        order_type: OrderType,
        currency: String,
        original_amount: Decimal,
        day_period: i32,
        id: String,
        timestamp: Option<DateTime<Utc>>,
    ) -> Self {
        LoanOrder {
            order_type,
            currency,
            original_amount,
            day_period,
            id,
            timestamp,
        }
    }
}

impl fmt::Display for LoanOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LoanOrder [type={:?}, currency={}, originalAmount={}, dayPeriod={}, id={}, timestamp={:?}]",
            self.order_type,
            self.currency,
            self.original_amount,
            self.day_period,
            self.id,
            self.timestamp
        )
    }
}
