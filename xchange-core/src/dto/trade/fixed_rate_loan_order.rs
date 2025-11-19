use crate::dto::loan_order::LoanOrder;
use crate::dto::order::OrderType;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// Represents a fixed rate loan order with a specified rate of return.
/// This struct is part of a loan management system where fixed rate loans
/// are offered with a defined rate for a specific period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedRateLoanOrder {
    /// The base loan order, assumed to be a part of the system's structure.
    /// Additional fields from the LoanOrder struct would go here.
    pub loan_order: LoanOrder,

    /// The fixed rate of return for the loan order (e.g., annual interest rate).
    /// This is stored using a `Decimal` type to support high precision.
    pub rate: Decimal,
}

impl FixedRateLoanOrder {
    /// Constructor for creating a new `FixedRateLoanOrder`.
    ///
    /// # Parameters
    /// - `rate`: The fixed rate of return for the loan order.
    /// - `order_type`: The type of order (Bid or Ask).
    /// - `currency`: The currency of the loan.
    /// - `original_amount`: The amount to be ordered or that was ordered.
    /// - `day_period`: Duration of the loan in days.
    /// - `id`: A unique identifier for the order.
    /// - `timestamp`: The timestamp of the order, `None` if not provided.
    ///
    /// # Returns
    /// A new instance of `FixedRateLoanOrder`.
    pub fn new(
        order_type: OrderType,
        currency: String,
        original_amount: Decimal,
        day_period: i32,
        id: String,
        timestamp: Option<DateTime<Utc>>,
        rate: Decimal,
    ) -> Self {
        FixedRateLoanOrder {
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

impl PartialEq for FixedRateLoanOrder {
    fn eq(&self, other: &Self) -> bool {
        // Directly compare LoanOrder and rate fields
        self.loan_order == other.loan_order && self.rate == other.rate
    }
}

impl Eq for FixedRateLoanOrder {}

impl PartialOrd for FixedRateLoanOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Compare the rate first, and if equal, compare the day_period
        self.rate.partial_cmp(&other.rate).or_else(|| {
            self.loan_order
                .day_period
                .partial_cmp(&other.loan_order.day_period)
        })
    }
}

impl Ord for FixedRateLoanOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare the rate first, and if equal, compare the day_period
        self.rate
            .cmp(&other.rate)
            .then_with(|| self.loan_order.day_period.cmp(&other.loan_order.day_period))
    }
}

impl Hash for FixedRateLoanOrder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the LoanOrder fields (e.g., id, currency, etc.)
        self.loan_order.hash(state);

        // Hash the rate field (mixing is done by Hasher automatically)
        self.rate.hash(state);
    }
}
