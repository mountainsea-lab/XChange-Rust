use crate::dto::trade::fixed_rate_loan_order::FixedRateLoanOrder;
use crate::dto::trade::floating_rate_loan_order::FloatingRateLoanOrder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents open loan orders. These are loan orders that have been placed with the exchange but have not yet been matched to a counterparty.
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenLoanOrders {
    /// The list of open fixed rate loan orders.
    pub open_fixed_rate_loan_orders: Vec<FixedRateLoanOrder>,

    /// The list of open floating rate loan orders.
    pub open_floating_rate_loan_orders: Vec<FloatingRateLoanOrder>,
}

impl OpenLoanOrders {
    /// Constructor for creating a new `OpenLoanOrders` instance.
    ///
    /// # Parameters
    /// - `open_fixed_rate_loan_orders`: A list of fixed rate loan orders.
    /// - `open_floating_rate_loan_orders`: A list of floating rate loan orders.
    ///
    /// # Returns
    /// A new instance of `OpenLoanOrders`.
    pub fn new(
        open_fixed_rate_loan_orders: Vec<FixedRateLoanOrder>,
        open_floating_rate_loan_orders: Vec<FloatingRateLoanOrder>,
    ) -> Self {
        OpenLoanOrders {
            open_fixed_rate_loan_orders,
            open_floating_rate_loan_orders,
        }
    }
}

// Implementing `Debug` trait to provide a similar `toString` behavior as Java's `toString`.
impl fmt::Display for OpenLoanOrders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OpenLoanOrders [openFixedRateLoanOrders={:?}, openFloatingRateLoanOrders={:?}]",
            self.open_fixed_rate_loan_orders, self.open_floating_rate_loan_orders
        )
    }
}
