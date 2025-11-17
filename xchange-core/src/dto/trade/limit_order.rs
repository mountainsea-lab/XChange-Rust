use crate::dto::order::{OrderBase, OrderStatus, OrderType};
use crate::instrument::InstrumentDTO;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;

/// DTO representing a limit order
///
///  <p>A limit order lets you set a minimum or maximum price before your trade will be treated by the
///  exchange as a {@link MarketOrder}. There is no guarantee that your conditions will be met on the
///  exchange, so your order may not be executed. However, until you become very experienced, almost
///  all orders should be limit orders to protect yourself.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitOrder {
    // Inherit from OrderBase
    #[serde(flatten)]
    pub order_base: OrderBase,

    // The limit price for the order
    pub limit_price: Option<Decimal>,
}

impl LimitOrder {
    // Constructor for a basic limit order (similar to the first Java constructor)
    pub fn new(
        type_: OrderType,
        original_amount: Option<Decimal>,
        instrument: InstrumentDTO,
        id: String,
        timestamp: Option<DateTime<Utc>>,
        limit_price: Option<Decimal>,
    ) -> Self {
        let order_base = OrderBase::new(
            type_,
            original_amount,
            instrument,
            id,
            None, // No user_reference
            timestamp,
            Some(OrderStatus::PendingNew),
        );
        LimitOrder {
            order_base,
            limit_price,
        }
    }

    // Constructor for a limit order with cumulative amount (like the second Java constructor)
    pub fn with_cumulative_amount(
        type_: OrderType,
        original_amount: Option<Decimal>,
        cumulative_amount: Decimal,
        instrument: InstrumentDTO,
        id: String,
        timestamp: Option<DateTime<Utc>>,
        limit_price: Option<Decimal>,
    ) -> Self {
        let order_base = OrderBase::with_details(
            type_,
            original_amount,
            instrument,
            id,
            None, // No user_reference
            timestamp,
            Some(OrderStatus::PendingNew),
            Some(cumulative_amount),
            None,           // No average_price
            None,           // No fee
            None,           // No leverage
            HashSet::new(), // No flags
        );
        LimitOrder {
            order_base,
            limit_price,
        }
    }

    // Constructor for a limit order with filled data (similar to the third Java constructor)
    pub fn with_filled_data(
        type_: OrderType,
        original_amount: Option<Decimal>,
        instrument: InstrumentDTO,
        id: String,
        timestamp: Option<DateTime<Utc>>,
        limit_price: Option<Decimal>,
        average_price: Decimal,
        cumulative_amount: Decimal,
        fee: Decimal,
        status: OrderStatus,
    ) -> Self {
        let order_base = OrderBase::with_details(
            type_,
            original_amount,
            instrument,
            id,
            None, // No user_reference
            timestamp,
            Some(status),
            Some(cumulative_amount),
            Some(average_price),
            Some(fee),
            None,           // No leverage
            HashSet::new(), // No flags
        );
        LimitOrder {
            order_base,
            limit_price,
        }
    }

    // Constructor for a limit order with user reference (similar to the fourth Java constructor)
    pub fn with_user_reference(
        type_: OrderType,
        original_amount: Option<Decimal>,
        instrument: InstrumentDTO,
        id: String,
        timestamp: Option<DateTime<Utc>>,
        limit_price: Option<Decimal>,
        average_price: Decimal,
        cumulative_amount: Decimal,
        fee: Decimal,
        status: Option<OrderStatus>,
        user_reference: String,
    ) -> Self {
        let order_base = OrderBase::with_details(
            type_,
            original_amount,
            instrument,
            id,
            Some(user_reference),
            timestamp,
            status,
            Some(cumulative_amount),
            Some(average_price),
            Some(fee),
            None,           // No leverage
            HashSet::new(), // No flags
        );
        LimitOrder {
            order_base,
            limit_price,
        }
    }

    pub fn get_limit_price(&self) -> Option<Decimal> {
        self.limit_price
    }

    pub fn print_limit_price(&self) -> Option<String> {
        self.limit_price.map(|p| p.to_string())
    }
}

impl fmt::Display for LimitOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LimitOrder [limitPrice={}, {}]",
            self.print_limit_price().unwrap_or("".to_string()),
            self.order_base
        )
    }
}

impl PartialEq for LimitOrder {
    fn eq(&self, other: &Self) -> bool {
        // compare limit_price as numeric BigDecimal.compareTo()
        let price_eq = match (self.limit_price, other.limit_price) {
            (Some(a), Some(b)) => a == b,
            (None, None) => true,
            _ => false,
        };

        price_eq && self.order_base == other.order_base
    }
}

impl Eq for LimitOrder {}

impl std::hash::Hash for LimitOrder {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.order_base.hash(state);
        if let Some(price) = self.limit_price {
            price.hash(state);
        }
    }
}

impl PartialOrd for LimitOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LimitOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;

        let t1 = self.order_base.type_.clone();
        let t2 = other.order_base.type_.clone();

        // If BID vs ASK → BID always < ASK
        if t1 != t2 {
            return if t1 == OrderType::Bid { Less } else { Greater };
        }

        // Same side
        match (self.limit_price, other.limit_price) {
            (Some(a), Some(b)) => {
                match t1 {
                    OrderType::Bid => b.cmp(&a),     // descending
                    OrderType::Ask => a.cmp(&b),     // ascending
                    OrderType::ExitAsk => b.cmp(&a), // descending
                    OrderType::ExitBid => a.cmp(&b), // ascending
                }
            }
            (None, None) => Equal,
            (Some(_), None) => Greater, // Java BigDecimal.compareTo(null) 不存在，但保持 deterministic
            (None, Some(_)) => Less,
        }
    }
}
