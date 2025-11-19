use crate::dto::order::OrderType;
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
    pub timestamp: Option<DateTime<Utc>>,
}

impl LoanOrderBook {
    pub fn new(
        fixed_rate_asks: Vec<FixedRateLoanOrder>,
        fixed_rate_bids: Vec<FixedRateLoanOrder>,
        floating_rate_asks: Vec<FloatingRateLoanOrder>,
        floating_rate_bids: Vec<FloatingRateLoanOrder>,
        timestamp: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            fixed_rate_asks,
            fixed_rate_bids,
            floating_rate_asks,
            floating_rate_bids,
            timestamp,
        }
    }

    /// Update logic for FixedRateLoanOrder
    pub fn update_fixed(&mut self, updated: FixedRateLoanOrder) {
        let lo = updated.loan_order.clone();

        let order_type = lo.order_type;
        let day_period = lo.day_period;
        let rate = updated.rate;
        let ts = lo.timestamp.clone();

        let list = match order_type {
            OrderType::Ask => &mut self.fixed_rate_asks,
            OrderType::Bid => &mut self.fixed_rate_bids,
            _ => return,
        };

        if let Some(pos) = list
            .iter()
            .position(|o| o.rate == rate && o.loan_order.day_period == day_period)
        {
            list.remove(pos);
        }

        list.push(updated);
        list.sort();

        self.timestamp = ts;
    }

    /// Update logic for FloatingRateLoanOrder
    pub fn update_floating(&mut self, updated: FloatingRateLoanOrder) {
        let lo = updated.loan_order.clone();

        let order_type = lo.order_type;
        let day_period = lo.day_period;
        let updated_rate = updated.rate;
        let ts = lo.timestamp.clone();

        // Select the target list based on order type (Ask or Bid)
        let list = match order_type {
            OrderType::Ask => &mut self.floating_rate_asks,
            OrderType::Bid => &mut self.floating_rate_bids,
            _ => return,
        };

        // Remove the first order that matches the day_period
        // Also check if the rate has changed
        let mut rate_changed = false;

        if let Some(pos) = list
            .iter()
            .position(|o| o.loan_order.day_period == day_period)
        {
            // Check if the existing rate differs from the updated one
            if list[pos].rate != updated_rate {
                rate_changed = true;
            }
            // Remove the first matching order
            list.remove(pos);
        }

        // Add the new updated order
        list.push(updated);
        // Sort the list (stable sort)
        list.sort();

        // If the rate has changed, update all orders in both Ask and Bid lists
        if rate_changed {
            for order in &mut self.floating_rate_asks {
                order.rate = updated_rate;
            }
            for order in &mut self.floating_rate_bids {
                order.rate = updated_rate;
            }
        }

        // Update the timestamp
        self.timestamp = ts;
    }
}
