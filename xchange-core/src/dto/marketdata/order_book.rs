use crate::dto::marketdata::order_book_update::OrderBookUpdate;
use crate::dto::order::OrderType;
use crate::dto::trade::limit_order::LimitOrder;
use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBook {
    #[serde(skip)]
    pub lock: Arc<RwLock<()>>, // Java 的 @JsonIgnore + final StampedLock 替代品

    pub asks: Vec<LimitOrder>,
    pub bids: Vec<LimitOrder>,
    pub timestamp: Option<DateTime<Utc>>,
}

impl Clone for OrderBook {
    fn clone(&self) -> Self {
        Self {
            // keep key share
            lock: Arc::clone(&self.lock),
            asks: self.asks.clone(),
            bids: self.bids.clone(),
            timestamp: self.timestamp,
        }
    }
}

impl OrderBook {
    /// ---- Equivalent to Java:
    ///  public OrderBook(timeStamp, asks, bids)
    pub fn new(
        timestamp: Option<DateTime<Utc>>,
        asks: Vec<LimitOrder>,
        bids: Vec<LimitOrder>,
    ) -> Self {
        Self::new_internal(timestamp, asks, bids, false)
    }

    /// ---- Equivalent to Java:
    ///  public OrderBook(timeStamp, asks, bids, sort)
    pub fn new_sorted(
        timestamp: Option<DateTime<Utc>>,
        asks: Vec<LimitOrder>,
        bids: Vec<LimitOrder>,
        sort: bool,
    ) -> Self {
        Self::new_internal(timestamp, asks, bids, sort)
    }

    /// Java 两个构造函数都通过这个逻辑归一化
    fn new_internal(
        timestamp: Option<DateTime<Utc>>,
        mut asks: Vec<LimitOrder>,
        mut bids: Vec<LimitOrder>,
        sort: bool,
    ) -> Self {
        if sort {
            asks.sort();
            bids.sort();
        }
        Self {
            lock: Arc::new(RwLock::new(())),
            timestamp,
            asks,
            bids,
        }
    }

    /// ---- Equivalent to Java:
    ///  public OrderBook(timeStamp, Stream<LimitOrder> asks, Stream<LimitOrder> bids)
    pub fn from_iter<I1, I2>(timestamp: Option<DateTime<Utc>>, asks: I1, bids: I2) -> Self
    where
        I1: IntoIterator<Item = LimitOrder>,
        I2: IntoIterator<Item = LimitOrder>,
    {
        Self::from_iter_sorted(timestamp, asks, bids, false)
    }

    /// ---- Equivalent to Java:
    /// public OrderBook(timeStamp, Stream<LimitOrder> asks, Stream<LimitOrder> bids, boolean sort)
    pub fn from_iter_sorted<I1, I2>(
        timestamp: Option<DateTime<Utc>>,
        asks: I1,
        bids: I2,
        sort: bool,
    ) -> Self
    where
        I1: IntoIterator<Item = LimitOrder>,
        I2: IntoIterator<Item = LimitOrder>,
    {
        let mut asks: Vec<_> = asks.into_iter().collect();
        let mut bids: Vec<_> = bids.into_iter().collect();

        if sort {
            asks.sort();
            bids.sort();
        }

        Self {
            lock: Arc::new(RwLock::new(())),
            timestamp,
            asks,
            bids,
        }
    }

    /// Returns a copy of limitOrder with tradeableAmount replaced.
    pub fn with_amount(order: &LimitOrder, tradeable_amount: Decimal) -> LimitOrder {
        let mut order_base = order.order_base.clone();
        order_base.original_amount = Some(tradeable_amount);

        LimitOrder {
            order_base,
            limit_price: order.limit_price,
        }
    }

    pub fn orders(&self, order_type: OrderType) -> &Vec<LimitOrder> {
        if order_type.is_ask() {
            &self.asks
        } else {
            &self.bids
        }
    }

    /// Given a new `LimitOrder`, this will replace a matching limit order in the order book
    /// if one is found, or add the new `LimitOrder` if none is found.
    /// The `timestamp` of the order book will be updated if the new timestamp
    /// is `Some` and later than the current timestamp.
    ///
    /// # Arguments
    ///
    /// * `limit_order` - The new `LimitOrder` to insert or update in the order book.
    pub fn update_with_limit_order(&mut self, limit_order: LimitOrder) {
        /// Acquire a write lock to ensure thread-safe access while updating the order book.
        let _guard = self.lock.write();

        // 取出对应的 orders 列表
        // let orders = self.orders_mut(limit_order.order_base.type_.clone());
        // 直接选择对应 orders，避免调用 self.orders_mut() 导致 borrow 冲突
        let orders = match limit_order.order_base.type_ {
            OrderType::Ask | OrderType::ExitAsk => &mut self.asks,
            OrderType::Bid | OrderType::ExitBid => &mut self.bids,
        };

        /// Perform a binary search on the orders to determine the correct insertion index.
        let mut idx = orders.binary_search(&limit_order).unwrap_or_else(|i| i);

        /// Remove the existing order if it matches the new limit order.
        if idx < orders.len() && orders.get(idx).map(|o| o == &limit_order).unwrap_or(false) {
            orders.remove(idx);
        }

        /// Insert the new `LimitOrder` into the orders if its remaining amount is non-zero.
        if let Some(remaining) = limit_order.order_base.original_amount {
            if remaining != Decimal::ZERO {
                orders.insert(idx, limit_order.clone());
            }
        }

        /// Update the order book timestamp, ensuring it only moves forward if the new timestamp is later.
        if let Some(ts) = limit_order.order_base.timestamp {
            if self.timestamp.map_or(true, |current| ts > current) {
                self.timestamp = Some(ts);
            }
        }
    }

    /// Given an `OrderBookUpdate`, replace a matching limit order in the order book if found,
    /// or insert a new one if not. The order book timestamp will be updated if the update timestamp
    /// is later than the current.
    pub fn update_with_order_book(&mut self, order_book_update: OrderBookUpdate) {
        /// Acquire a write lock to ensure thread-safe access while updating the order book.
        let _guard = self.lock.write();

        let limit_order = order_book_update.limit_order;
        let orders = match limit_order.order_base.type_ {
            OrderType::Ask | OrderType::ExitAsk => &mut self.asks,
            OrderType::Bid | OrderType::ExitBid => &mut self.bids,
        };

        /// Perform a binary search on the orders to determine the correct insertion index.
        let mut idx = orders.binary_search(&limit_order).unwrap_or_else(|i| i);

        ///  Remove the existing order if it matches the new limit order.
        if idx < orders.len() && orders.get(idx).map(|o| o == &limit_order).unwrap_or(false) {
            orders.remove(idx);
        }

        /// If the total volume is non-zero, insert a new `LimitOrder` where the amount is replaced by `total_volume`.
        if order_book_update.total_volume != Decimal::ZERO {
            let updated_order = Self::with_amount(&limit_order, order_book_update.total_volume);
            orders.insert(idx, updated_order);
        }

        /// Update the `OrderBook` timestamp to the new value if it is later than the current one.
        if let Some(ts) = limit_order.order_base.timestamp {
            if self.timestamp.map_or(true, |current| ts > current) {
                self.timestamp = Some(ts);
            }
        }
    }

    /// Returns true if we need to run binary search again.
    fn recheck_idx(limit_orders: &[LimitOrder], limit_order: &LimitOrder, idx: isize) -> bool {
        match idx {
            0 => {
                if !limit_orders.is_empty() {
                    // If the first order is not equal, need to recheck
                    limit_orders[0].cmp(limit_order) != std::cmp::Ordering::Equal
                } else {
                    true
                }
            }
            -1 => {
                if limit_orders.is_empty() {
                    false
                } else {
                    // If first order <= limit_order, need to recheck
                    limit_orders[0].cmp(limit_order) != std::cmp::Ordering::Greater
                }
            }
            _ => true,
        }
    }

    /// Replace the timestamp if the provided date is `Some` and in the future.
    ///
    /// # TODO
    /// Should this raise an error if the order timestamp is in the past?
    fn update_date(&mut self, update_date: Option<DateTime<Utc>>) {
        if let Some(update_ts) = update_date {
            if self.timestamp.map_or(true, |current| update_ts > current) {
                self.timestamp = Some(update_ts);
            }
        }
    }
}
