use crate::dto::loan_order::LoanOrder;
use crate::dto::order::OrderType;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Represents a floating rate loan order where the rate is determined by the market.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct FloatingRateLoanOrder {
    /// The base loan order containing essential order details.
    pub loan_order: LoanOrder,

    /// The floating rate for the loan order (e.g., market-determined rate).
    pub rate: Decimal,
}

impl FloatingRateLoanOrder {
    /// Constructor for creating a new `FloatingRateLoanOrder`.
    pub fn new(
        order_type: OrderType,
        currency: String,
        original_amount: Decimal,
        day_period: i32,
        id: String,
        timestamp: Option<DateTime<Utc>>,
        rate: Decimal,
    ) -> Self {
        FloatingRateLoanOrder {
            loan_order: LoanOrder::new(
                order_type,
                currency,
                original_amount,
                day_period,
                id,
                timestamp,
            ),
            rate,
        }
    }
}

impl PartialOrd for FloatingRateLoanOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Compare by `day_period` from `LoanOrder`
        self.loan_order
            .day_period
            .partial_cmp(&other.loan_order.day_period)
    }
}

impl Ord for FloatingRateLoanOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare by `day_period` from `LoanOrder`
        self.loan_order.day_period.cmp(&other.loan_order.day_period)
    }
}
