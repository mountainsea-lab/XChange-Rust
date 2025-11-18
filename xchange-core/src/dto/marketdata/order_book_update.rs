use crate::dto::order::OrderType;
use crate::dto::trade::limit_order::LimitOrder;
use crate::instrument::InstrumentDTO;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

/// Immutable data object representing a Market Depth update.
#[derive(Debug, Clone)]
pub struct OrderBookUpdate {
    /// The limit order associated with this update.
    pub limit_order: LimitOrder,

    /// The total volume at this price in the order book (in base currency).
    pub total_volume: Decimal,
}

impl OrderBookUpdate {
    /// Build an order book update.
    ///
    /// # Arguments
    ///
    /// * `order_type` - The order type (Bid/Ask/ExitBid/ExitAsk)
    /// * `volume` - Volume of the limit order in the base currency (e.g., BTC for BTC/USD)
    /// * `instrument` - The instrument traded (e.g., BTC/USD)
    /// * `limit_price` - Price of this update in counter currency per base currency (e.g., $/BTC)
    /// * `timestamp` - Timestamp for the update
    /// * `total_volume` - Total new volume of open orders for this price in the order book
    pub fn new(
        order_type: OrderType,
        volume: Decimal,
        instrument: InstrumentDTO,
        limit_price: Decimal,
        timestamp: Option<DateTime<Utc>>,
        total_volume: Decimal,
    ) -> Self {
        let limit_order = LimitOrder::new(
            order_type,
            Some(volume),
            instrument,
            "".to_string(), // empty id
            timestamp,
            Some(limit_price),
        );

        Self {
            limit_order,
            total_volume,
        }
    }

    /// Get a reference to the limit order.
    pub fn limit_order(&self) -> &LimitOrder {
        &self.limit_order
    }

    /// Get the total volume at this price.
    pub fn total_volume(&self) -> Decimal {
        self.total_volume
    }
}

impl std::fmt::Display for OrderBookUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OrderBookUpdate [limit_order={:?}, total_volume={}]",
            self.limit_order, self.total_volume
        )
    }
}
