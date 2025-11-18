use crate::dto::order::OrderType;
use crate::instrument::InstrumentDTO;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trade {
    /// Did this trade result from the execution of a bid or an ask?
    pub order_type: OrderType, // `type` is a reserved keyword, so we use `order_type`

    /// Amount that was traded
    pub original_amount: Decimal,

    /// The instrument
    pub instrument: InstrumentDTO,

    /// The price of the trade
    pub price: Decimal,

    /// The timestamp of the trade according to the exchange's server, null if not provided
    pub timestamp: Option<DateTime<Utc>>,

    /// The trade id
    pub id: String,

    /// The maker order id
    pub maker_order_id: String,

    /// The taker order id
    pub taker_order_id: String,
}
