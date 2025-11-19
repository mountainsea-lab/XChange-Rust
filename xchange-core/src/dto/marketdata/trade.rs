use crate::dto::order::OrderType;
use crate::instrument::InstrumentDTO;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Data object representing a Trade
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

pub struct TradeBuilder {
    order_type: Option<OrderType>,
    original_amount: Option<Decimal>,
    instrument: Option<InstrumentDTO>,
    price: Option<Decimal>,
    timestamp: Option<DateTime<Utc>>,
    id: Option<String>,
    maker_order_id: Option<String>,
    taker_order_id: Option<String>,
}

impl TradeBuilder {
    // Constructor to initialize the builder
    pub fn new() -> Self {
        TradeBuilder {
            order_type: None,
            original_amount: None,
            instrument: None,
            price: None,
            timestamp: None,
            id: None,
            maker_order_id: None,
            taker_order_id: None,
        }
    }

    // Builder pattern methods for each field
    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = Some(order_type);
        self
    }

    pub fn original_amount(mut self, original_amount: Decimal) -> Self {
        self.original_amount = Some(original_amount);
        self
    }

    pub fn instrument(mut self, instrument: Option<InstrumentDTO>) -> Self {
        self.instrument = instrument;
        self
    }

    pub fn price(mut self, price: Decimal) -> Self {
        self.price = Some(price);
        self
    }

    pub fn timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn maker_order_id(mut self, maker_order_id: String) -> Self {
        self.maker_order_id = Some(maker_order_id);
        self
    }

    pub fn taker_order_id(mut self, taker_order_id: String) -> Self {
        self.taker_order_id = Some(taker_order_id);
        self
    }

    // Build the final Trade object
    pub fn build(self) -> Result<Trade, &'static str> {
        // Ensure that all required fields are set
        let order_type = self.order_type.ok_or("Missing required field: type")?;
        let original_amount = self
            .original_amount
            .ok_or("Missing required field: original_amount")?;
        let instrument = self
            .instrument
            .ok_or("Missing required field: instrument")?;
        let price = self.price.ok_or("Missing required field: price")?;
        let timestamp = self.timestamp.ok_or("Missing required field: timestamp")?;
        let id = self.id.ok_or("Missing required field: id")?;

        // Return the built Trade object
        Ok(Trade {
            order_type,
            original_amount,
            instrument,
            price,
            timestamp: Some(timestamp),
            id,
            maker_order_id: self.maker_order_id.unwrap_or_default(),
            taker_order_id: self.taker_order_id.unwrap_or_default(),
        })
    }
}

impl Trade {
    // Convenience method to create a builder from an existing Trade object (like the `from` method in Java)
    pub fn builder_from(trade: &Trade) -> TradeBuilder {
        TradeBuilder::new()
            .order_type(trade.order_type.clone())
            .original_amount(trade.original_amount)
            .instrument(Some(trade.instrument.clone()))
            .price(trade.price)
            .timestamp(trade.timestamp.clone().unwrap_or_default()) // Assuming timestamp is Some, else Default NaiveDateTime
            .id(trade.id.clone())
            .maker_order_id(trade.maker_order_id.clone())
            .taker_order_id(trade.taker_order_id.clone())
    }
}
