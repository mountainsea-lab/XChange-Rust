use crate::dto::order::{OrderBase, OrderStatus, OrderType};
use crate::instrument::InstrumentDTO;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
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
}
