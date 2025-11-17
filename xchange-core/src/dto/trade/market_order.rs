use crate::dto::order::{OrderBase, OrderStatus, OrderType};
use crate::instrument::InstrumentDTO;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

///  DTO representing a market order
///
///   <p>A market order is a buy or sell order to be executed immediately at current market prices. As
///   long as there are willing sellers and buyers, market orders are filled. Market orders are
///   therefore used when certainty of execution is a priority over price of execution. <strong>Use
///   market orders with caution, and review {@link LimitOrder} in case it is more suitable.</strong>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketOrder {
    pub order_base: OrderBase,
}

impl MarketOrder {
    pub fn new(
        order_type: OrderType,
        original_amount: Decimal,
        instrument: InstrumentDTO,
        id: String,
        timestamp: Option<DateTime<Utc>>,
        average_price: Option<Decimal>,
        cumulative_amount: Option<Decimal>,
        fee: Option<Decimal>,
        status: OrderStatus,
        user_reference: Option<String>, // 可选字段
    ) -> Self {
        let order_base = OrderBase {
            type_: order_type,
            original_amount: Some(original_amount),
            instrument,
            id,
            user_reference,
            timestamp,
            order_flags: HashSet::new(),
            status: Some(status),
            cumulative_amount,
            remaining_amount: None,
            average_price,
            fee,
            leverage: None,
        };

        MarketOrder { order_base }
    }
}

pub struct MarketOrderBuilder {
    order_base: OrderBase,
}

impl MarketOrderBuilder {
    pub fn new(order_type: OrderType, instrument: InstrumentDTO, id: String) -> Self {
        Self {
            order_base: OrderBase::new(order_type, None, instrument, id, None, None, None),
        }
    }

    pub fn original_amount(&mut self, amt: Decimal) -> &mut Self {
        self.order_base.original_amount = Some(amt);
        self
    }

    pub fn cumulative_amount(&mut self, amt: Decimal) -> &mut Self {
        self.order_base.cumulative_amount = Some(amt);
        self
    }

    pub fn average_price(&mut self, price: Decimal) -> &mut Self {
        self.order_base.average_price = Some(price);
        self
    }

    pub fn fee(&mut self, fee: Decimal) -> &mut Self {
        self.order_base.fee = Some(fee);
        self
    }

    pub fn timestamp(&mut self, ts: DateTime<Utc>) -> &mut Self {
        self.order_base.timestamp = Some(ts);
        self
    }

    pub fn status(&mut self, status: OrderStatus) -> &mut Self {
        self.order_base.status = Some(status);
        self
    }

    pub fn user_reference(&mut self, user_ref: String) -> &mut Self {
        self.order_base.user_reference = Some(user_ref);
        self
    }

    pub fn build(&self) -> MarketOrder {
        let mut base = self.order_base.clone();
        MarketOrder { order_base: base }
    }
}
