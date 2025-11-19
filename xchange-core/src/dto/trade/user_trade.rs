use crate::currency::currency::Currency;
use crate::dto::BuildError;
use crate::dto::marketdata::trade::Trade;
use crate::dto::order::OrderType;
use crate::instrument::{Instrument, InstrumentDTO, InstrumentKind};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Data object representing a user trade, extending the `Trade` struct
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserTrade {
    /// The base trade information (this will include fields from Trade)
    pub trade: Trade,

    /// The id of the order responsible for execution of this trade
    pub order_id: String,

    /// The fee that was charged by the exchange for this trade
    pub fee_amount: Decimal,

    /// The currency in which the fee was charged
    pub fee_currency: Currency,

    /// The order reference id which has been added by the user on the order creation
    pub order_user_reference: String,
}

impl UserTrade {
    /// Constructor for creating a new `UserTrade`
    pub fn new(
        trade: Trade,
        order_id: String,
        fee_amount: Decimal,
        fee_currency: Currency,
        order_user_reference: String,
    ) -> Self {
        UserTrade {
            trade,
            order_id,
            fee_amount,
            fee_currency,
            order_user_reference,
        }
    }

    pub fn builder() -> UserTradeBuilder {
        UserTradeBuilder::default()
    }
}

/// Implementing `Display` trait for UserTrade to provide a string representation similar to Java's `toString()`
impl fmt::Display for UserTrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let instrument_kind: InstrumentKind = self.trade.instrument.clone().into();
        write!(
            f,
            "UserTrade[type={:?}, originalAmount={}, instrument={:?}, price={}, timestamp={:?}, id={}, orderId='{}', feeAmount={}, feeCurrency='{:#?}', orderUserReference='{}']",
            self.trade.order_type,
            self.trade.original_amount,
            instrument_kind.symbol(),
            self.trade.price,
            self.trade.timestamp,
            self.trade.id,
            self.order_id,
            self.fee_amount,
            self.fee_currency,
            self.order_user_reference
        )
    }
}

#[derive(Debug, Default)]
pub struct UserTradeBuilder {
    order_id: Option<String>,
    fee_amount: Option<Decimal>,
    fee_currency: Option<Currency>,
    order_user_reference: Option<String>,
    order_type: Option<OrderType>,
    original_amount: Option<Decimal>,
    instrument: Option<InstrumentDTO>,
    price: Option<Decimal>,
    timestamp: Option<DateTime<Utc>>,
    id: Option<String>,
}

impl UserTradeBuilder {
    pub fn order_id(mut self, order_id: String) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn fee_amount(mut self, fee_amount: Decimal) -> Self {
        self.fee_amount = Some(fee_amount);
        self
    }

    pub fn fee_currency(mut self, fee_currency: Currency) -> Self {
        self.fee_currency = Some(fee_currency);
        self
    }

    pub fn order_user_reference(mut self, order_user_reference: String) -> Self {
        self.order_user_reference = Some(order_user_reference);
        self
    }

    // Inherited or shared fields from the base Trade class
    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = Some(order_type);
        self
    }

    pub fn original_amount(mut self, original_amount: Decimal) -> Self {
        self.original_amount = Some(original_amount);
        self
    }

    pub fn instrument(mut self, instrument: InstrumentDTO) -> Self {
        self.instrument = Some(instrument);
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

    pub fn build(self) -> Result<UserTrade, BuildError> {
        let trade = Trade::new(
            self.order_type
                .ok_or(BuildError::MissingField("order_type".into()))?,
            self.original_amount
                .ok_or(BuildError::MissingField("original_amount".into()))?,
            self.instrument
                .ok_or(BuildError::MissingField("instrument".into()))?,
            self.price.ok_or(BuildError::MissingField("price".into()))?,
            self.timestamp.map(Into::into),
            self.id.ok_or(BuildError::MissingField("id".into()))?,
            "".to_string(),
            "".to_string(),
        );

        Ok(UserTrade::new(
            trade,
            self.order_id
                .ok_or(BuildError::MissingField("order_id".into()))?,
            self.fee_amount
                .ok_or(BuildError::MissingField("fee_amount".into()))?,
            self.fee_currency
                .ok_or(BuildError::MissingField("fee_currency".into()))?,
            self.order_user_reference
                .ok_or(BuildError::MissingField("order_user_reference".into()))?,
        ))
    }
}
