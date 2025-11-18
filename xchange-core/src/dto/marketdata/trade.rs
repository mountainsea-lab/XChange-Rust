use crate::dto::order::OrderType;
use crate::instrument::InstrumentDTO;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Data object representing a Trade
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

impl Trade {
    /// This constructor is called to create a public `Trade` object in `MarketDataService::get_trades`.
    /// It is used when the `orderId` and `fee` parameters are not provided.
    ///
    /// # Parameters
    /// - `r#type`: The trade type (BID side or ASK side).
    /// - `original_amount`: The depth of this trade (amount traded).
    /// - `price`: The price of the trade (either the bid or the ask).
    /// - `timestamp`: The timestamp of the trade according to the exchange's server, `None` if not provided.
    /// - `id`: The id of the trade.
    /// - `maker_order_id`: The order id of the maker in the trade.
    /// - `taker_order_id`: The order id of the taker in the trade.
    pub fn new(
        order_type: OrderType,
        original_amount: Decimal,
        instrument: InstrumentDTO,
        price: Decimal,
        timestamp: Option<DateTime<Utc>>,
        id: String,
        maker_order_id: String,
        taker_order_id: String,
    ) -> Self {
        Trade {
            order_type,
            original_amount,
            instrument,
            price,
            timestamp,
            id,
            maker_order_id,
            taker_order_id,
        }
    }
}
