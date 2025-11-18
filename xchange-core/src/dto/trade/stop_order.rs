use crate::dto::order::{OrderBase, OrderStatus, OrderType};
use crate::instrument::InstrumentDTO;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
enum Intention {
    StopLoss,
    TakeProfit,
}

///  DTO representing a stop order
///
///   <p>A stop order lets you set a minimum or maximum price before your trade will be treated by the
///   exchange as a {@link MarketOrder} unless a limit price is also set. There is no guarantee that
///   your conditions will be met on the exchange, so your order may not be executed.
#[derive(Debug, Serialize, Deserialize)]
pub struct StopOrder {
    // Including OrderBase as a field
    pub base: OrderBase,

    // Stop price is always required
    pub stop_price: Decimal,

    // Limit price is optional
    pub limit_price: Option<Decimal>,

    // Intention is optional (could be StopLoss or TakeProfit)
    pub intention: Option<Intention>,

    // TrailValue is optional
    pub trail_value: Option<Decimal>,
}

impl StopOrder {
    // Full associated constructor using Option for optional parameters
    pub fn new(
        type_: OrderType,
        original_amount: Option<Decimal>,
        instrument: InstrumentDTO,
        id: String,
        stop_price: Decimal,
        limit_price: Option<Decimal>,
        average_price: Option<Decimal>,
        cumulative_amount: Option<Decimal>,
        fee: Option<Decimal>,
        status: Option<OrderStatus>,
        user_reference: Option<String>,
        intention: Option<Intention>,
        trail_value: Option<Decimal>,
        timestamp: Option<DateTime<Utc>>,
    ) -> Self {
        StopOrder {
            base: OrderBase {
                type_: type_,
                original_amount,
                instrument,
                id,
                user_reference,
                timestamp,
                order_flags: HashSet::new(), // Default empty set
                status,
                cumulative_amount,
                remaining_amount: None, // Assuming remaining_amount starts as None
                average_price,
                fee,
                leverage: None, // Assuming leverage is optional and can be None
            },
            stop_price,
            limit_price,
            intention,
            trail_value,
        }
    }

    pub fn stop_price(&self) -> &Decimal {
        &self.stop_price
    }

    pub fn limit_price(&self) -> Option<&Decimal> {
        self.limit_price.as_ref()
    }

    pub fn intention(&self) -> Option<&Intention> {
        self.intention.as_ref()
    }

    pub fn trail_value(&self) -> Option<&Decimal> {
        self.trail_value.as_ref()
    }
}

// Implementing the `fmt::Display` trait for user-friendly string representation
impl fmt::Display for StopOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StopOrder {{ stop_price: {}, limit_price: {:?}, intention: {:?}, trail_value: {:?} }}",
            self.stop_price, self.limit_price, self.intention, self.trail_value
        )
    }
}

// Implementing PartialEq for equality comparison
impl PartialEq for StopOrder {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
            && self.stop_price == other.stop_price
            && self.limit_price == other.limit_price
            && self.intention == other.intention
            && self.trail_value == other.trail_value
    }
}

impl Eq for StopOrder {}

// Implementing Hash for StopOrder
impl Hash for StopOrder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.hash(state);
        self.stop_price.hash(state);
        self.limit_price.hash(state);
        self.intention.hash(state);
        self.trail_value.hash(state);
    }
}

// Implementing Ord and PartialOrd for custom comparison logic
impl Ord for StopOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare the types (BID < ASK)
        if self.base.type_ != other.base.type_ {
            return if self.base.type_ == OrderType::Bid {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }

        // Compare the stopPrice (lower stopPrice comes first for BID, higher comes first for ASK)
        let stop_price_cmp = self.stop_price.cmp(&other.stop_price);
        if stop_price_cmp != Ordering::Equal {
            return if self.base.type_ == OrderType::Bid {
                stop_price_cmp
            } else {
                stop_price_cmp.reverse()
            };
        }
        // Compare the intention (StopLoss < TakeProfit for both types)
        self.intention.cmp(&other.intention)
    }
}

impl PartialOrd for StopOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
