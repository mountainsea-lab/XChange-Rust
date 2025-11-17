use crate::dto::trade::limit_order::LimitOrder;
use crate::instrument::InstrumentDTO;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    Bid,     // Buying order (the trader is providing the counter currency)
    Ask,     // Selling order (the trader is providing the base currency)
    ExitAsk, // Close a short position in crypto derivatives
    ExitBid, // Close a long position in crypto derivatives
}

impl OrderType {
    // Method to get the opposite of the order type
    pub fn get_opposite(self) -> Option<OrderType> {
        match self {
            OrderType::Bid => Some(OrderType::Ask),
            OrderType::Ask => Some(OrderType::Bid),
            OrderType::ExitAsk => Some(OrderType::ExitBid),
            OrderType::ExitBid => Some(OrderType::ExitAsk),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    PendingNew,        // Initial order when instantiated
    NEW,               // Initial order when placed on the order book at exchange
    PartiallyFilled,   // Partially match against opposite order on order book at exchange
    FILLED,            // Fully match against opposite order on order book at exchange
    PendingCancel,     // Waiting to be removed from order book at exchange
    PartiallyCanceled, // Order was partially canceled at exchange
    CANCELED,          // Removed from order book at exchange
    PendingReplace,    // Waiting to be replaced by another order on order book at exchange
    REPLACED,          // Order has been replaced by another order on order book at exchange
    STOPPED,           // Order has been triggered at stop price
    REJECTED,          // Order has been rejected by exchange and not placed on order book
    EXPIRED,           // Order has expired and been removed from order book
    OPEN,              // Order is open and waiting to be filled
    CLOSED,            // Order has been either filled or cancelled
    UNKNOWN,           // The exchange returned a state not in the documentation
}

impl OrderStatus {
    // Method to check if the order status is final
    pub fn is_final(self) -> bool {
        matches!(
            self,
            OrderStatus::FILLED
                | OrderStatus::PartiallyCanceled
                | OrderStatus::CANCELED
                | OrderStatus::REPLACED
                | OrderStatus::STOPPED
                | OrderStatus::REJECTED
                | OrderStatus::EXPIRED
                | OrderStatus::CLOSED
        )
    }

    // Method to check if the order status is open
    pub fn is_open(self) -> bool {
        matches!(
            self,
            OrderStatus::PendingNew
                | OrderStatus::NEW
                | OrderStatus::PartiallyFilled
                | OrderStatus::OPEN
        )
    }
}

// Define the trait or just use an enum to represent different types of flags.
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum OrderFlag {
    ImmediateOrCancel,
    FillOrKill,
    PostOnly,
}

impl OrderFlag {
    // You can define custom methods on the enum if needed
    pub fn description(&self) -> &str {
        match self {
            OrderFlag::ImmediateOrCancel => "Flag immediateOrCancel",
            OrderFlag::FillOrKill => "Flag Fill Or Kill",
            OrderFlag::PostOnly => "Flag Post Only",
        }
    }
}

// Trait to represent the shared behavior of different orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Order {
    LimitOrder(LimitOrder),
    // StopOrder(StopOrder),
    // MarketOrder(MarketOrder),
}

impl Order {
    // Example method to retrieve the ID
    pub fn id(&self) -> &str {
        match self {
            Order::LimitOrder(order) => &order.order_base.id,
            // Order::StopOrder(order) => &order.order_base.id,
            // Order::MarketOrder(order) => &order.order_base.id,
        }
    }
}

// Common struct to hold fields that are shared between all order types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBase {
    pub type_: OrderType,
    pub original_amount: Option<Decimal>,
    pub instrument: InstrumentDTO, // Could be a struct for Instrument if needed
    pub id: String,
    pub user_reference: Option<String>,
    pub timestamp: Option<DateTime<Utc>>,
    pub order_flags: HashSet<OrderFlag>,
    pub status: Option<OrderStatus>,
    pub cumulative_amount: Option<Decimal>,
    pub average_price: Option<Decimal>,
    pub fee: Option<Decimal>,
    pub leverage: Option<String>,
}

impl OrderBase {
    // Constructor for a basic order with minimal fields (no optional fields)
    pub fn new(
        type_: OrderType,
        original_amount: Option<Decimal>,
        instrument: InstrumentDTO,
        id: String,
        user_reference: Option<String>,
        timestamp: Option<DateTime<Utc>>,
        status: Option<OrderStatus>,
    ) -> Self {
        OrderBase {
            type_,
            original_amount,
            instrument,
            id,
            user_reference,
            timestamp,
            order_flags: HashSet::new(), // Default to an empty set
            status,
            cumulative_amount: None,
            average_price: None,
            fee: None,
            leverage: None,
        }
    }

    // Constructor for an order with optional fields provided
    pub fn with_details(
        type_: OrderType,
        original_amount: Option<Decimal>,
        instrument: InstrumentDTO,
        id: String,
        user_reference: Option<String>,
        timestamp: Option<DateTime<Utc>>,
        status: Option<OrderStatus>,
        cumulative_amount: Option<Decimal>,
        average_price: Option<Decimal>,
        fee: Option<Decimal>,
        leverage: Option<String>,
        order_flags: HashSet<OrderFlag>,
    ) -> Self {
        OrderBase {
            type_,
            original_amount,
            instrument,
            id,
            user_reference,
            timestamp,
            order_flags,
            status,
            cumulative_amount,
            average_price,
            fee,
            leverage,
        }
    }

    // Constructor to create an order with default values for optional fields
    pub fn with_defaults(
        type_: OrderType,
        original_amount: Option<Decimal>,
        instrument: InstrumentDTO,
        id: String,
        user_reference: Option<String>,
        timestamp: Option<DateTime<Utc>>,
        status: Option<OrderStatus>,
    ) -> Self {
        OrderBase {
            type_,
            original_amount,
            instrument,
            id,
            user_reference,
            timestamp,
            order_flags: HashSet::new(),
            status,
            cumulative_amount: None,
            average_price: None,
            fee: None,
            leverage: None,
        }
    }

    pub fn fee(&self) -> Option<Decimal> {
        self.fee
    }

    pub fn set_fee(&mut self, fee: Option<Decimal>) {
        self.fee = fee;
    }

    // ----------  getType ----------
    pub fn type_(&self) -> OrderType {
        self.type_.clone()
    }

    // ----------  getStatus / setOrderStatus ----------
    pub fn status(&self) -> Option<OrderStatus> {
        self.status
    }

    pub fn set_status(&mut self, status: OrderStatus) {
        self.status = Some(status);
    }

    // ----------  getOriginalAmount ----------
    pub fn original_amount(&self) -> Option<Decimal> {
        self.original_amount
    }

    // ----------  getCumulativeAmount / setCumulativeAmount ----------
    pub fn cumulative_amount(&self) -> Option<Decimal> {
        self.cumulative_amount
    }

    pub fn set_cumulative_amount(&mut self, value: Option<Decimal>) {
        self.cumulative_amount = value;
    }

    // ----------  getCumulativeCounterAmount ----------
    pub fn cumulative_counter_amount(&self) -> Option<Decimal> {
        if let (Some(cum), Some(price)) = (self.cumulative_amount, self.average_price) {
            if price > Decimal::ZERO {
                return Some(cum * price);
            }
        }
        None
    }

    // ---------- getRemainingAmount ----------
    pub fn remaining_amount(&self) -> Option<Decimal> {
        match (self.original_amount, self.cumulative_amount) {
            (Some(orig), Some(filled)) => Some(orig - filled),
            (Some(orig), None) => Some(orig),
            _ => None,
        }
    }

    // ----------  getAveragePrice / setAveragePrice ----------
    pub fn average_price(&self) -> Option<Decimal> {
        self.average_price
    }

    pub fn set_average_price(&mut self, price: Option<Decimal>) {
        self.average_price = price;
    }

    // ----------  getInstrument ----------
    pub fn instrument(&self) -> &InstrumentDTO {
        &self.instrument
    }

    // ----------  getId ----------
    pub fn id(&self) -> &str {
        &self.id
    }

    // ----------  getUserReference ----------
    pub fn user_reference(&self) -> Option<&String> {
        self.user_reference.as_ref()
    }

    // ----------  getTimestamp ----------
    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        self.timestamp
    }

    // ----------  getOrderFlags / setOrderFlags ----------
    pub fn order_flags(&self) -> &HashSet<OrderFlag> {
        &self.order_flags
    }

    pub fn set_order_flags(&mut self, flags: Option<HashSet<OrderFlag>>) {
        self.order_flags.clear();
        if let Some(f) = flags {
            self.order_flags.extend(f);
        }
    }

    // ----------  hasFlag / addOrderFlag ----------
    pub fn has_flag(&self, flag: OrderFlag) -> bool {
        self.order_flags.contains(&flag)
    }

    pub fn add_order_flag(&mut self, flag: OrderFlag) {
        self.order_flags.insert(flag);
    }

    // ----------  getLeverage / setLeverage ----------
    pub fn leverage(&self) -> Option<&String> {
        self.leverage.as_ref()
    }

    pub fn set_leverage(&mut self, lev: Option<String>) {
        self.leverage = lev;
    }

    pub fn builder(type_: OrderType, instrument: InstrumentDTO) -> OrderBaseBuilder {
        OrderBaseBuilder::new(type_, instrument)
    }
}

impl fmt::Display for OrderBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let print_decimal = |v: &Option<Decimal>| {
            v.as_ref()
                .map(|d| d.to_string())
                .unwrap_or_else(|| "null".into())
        };

        let print_str = |v: &Option<String>| v.clone().unwrap_or_else(|| "null".into());

        let print_dt = |v: &Option<DateTime<Utc>>| {
            v.as_ref()
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| "null".into())
        };

        write!(
            f,
            "Order [type={:?}, originalAmount={}, cumulativeAmount={}, averagePrice={}, \
fee={}, instrument={:?}, id={}, timestamp={}, status={:?}, flags={:?}, userReference={}]",
            self.type_,
            print_decimal(&self.original_amount.clone()),
            print_decimal(&self.cumulative_amount),
            print_decimal(&self.average_price),
            print_decimal(&self.fee),
            self.instrument,
            self.id,
            print_dt(&self.timestamp),
            self.status,
            self.order_flags,
            print_str(&self.user_reference),
        )
    }
}

impl PartialEq for OrderBase {
    fn eq(&self, other: &Self) -> bool {
        self.type_ == other.type_
            && self.original_amount.cmp(&other.original_amount) == std::cmp::Ordering::Equal
            && self.instrument == other.instrument
            && self.id == other.id
            && self.timestamp == other.timestamp
    }
}

impl Eq for OrderBase {}

impl Hash for OrderBase {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.type_.hash(state);
        self.original_amount.hash(state); // Decimal implements Hash
        self.instrument.hash(state);
        self.id.hash(state);
        self.timestamp.hash(state);
    }
}

#[derive(Debug)]
pub struct OrderBaseBuilder {
    pub(crate) type_: OrderType,
    pub(crate) original_amount: Option<Decimal>,
    pub(crate) cumulative_amount: Option<Decimal>,
    pub(crate) remaining_amount: Option<Decimal>,
    pub(crate) instrument: InstrumentDTO,
    pub(crate) id: String,
    pub(crate) user_reference: Option<String>,
    pub(crate) timestamp: Option<DateTime<Utc>>,
    pub(crate) average_price: Option<Decimal>,
    pub(crate) status: Option<OrderStatus>,
    pub(crate) fee: Option<Decimal>,
    pub(crate) leverage: Option<String>,
    pub(crate) order_flags: HashSet<OrderFlag>,
}

impl OrderBaseBuilder {
    pub fn new(type_: OrderType, instrument: InstrumentDTO) -> Self {
        Self {
            type_,
            instrument,
            original_amount: None,
            cumulative_amount: None,
            remaining_amount: None,
            id: "".to_string(),
            user_reference: None,
            timestamp: None,
            average_price: None,
            status: None,
            fee: None,
            leverage: None,
            order_flags: HashSet::new(),
        }
    }

    pub fn order_type(&mut self, type_: OrderType) -> &mut Self {
        self.type_ = type_;
        self
    }

    pub fn original_amount(&mut self, amount: Decimal) -> &mut Self {
        self.original_amount = Some(amount);
        self
    }

    pub fn cumulative_amount(&mut self, amount: Decimal) -> &mut Self {
        self.cumulative_amount = Some(amount);
        self
    }

    pub fn remaining_amount(&mut self, amount: Decimal) -> &mut Self {
        self.remaining_amount = Some(amount);
        self
    }

    pub fn instrument(&mut self, instrument: InstrumentDTO) -> &mut Self {
        self.instrument = instrument;
        self
    }

    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id = id.into();
        self
    }

    pub fn user_reference(&mut self, user_ref: impl Into<String>) -> &mut Self {
        self.user_reference = Some(user_ref.into());
        self
    }

    pub fn timestamp(&mut self, ts: DateTime<Utc>) -> &mut Self {
        self.timestamp = Some(ts);
        self
    }

    pub fn average_price(&mut self, price: Decimal) -> &mut Self {
        self.average_price = Some(price);
        self
    }

    pub fn status(&mut self, status: OrderStatus) -> &mut Self {
        self.status = Some(status);
        self
    }

    pub fn fee(&mut self, fee: Decimal) -> &mut Self {
        self.fee = Some(fee);
        self
    }

    pub fn leverage(&mut self, leverage: impl Into<String>) -> &mut Self {
        self.leverage = Some(leverage.into());
        self
    }

    pub fn flags(&mut self, flags: HashSet<OrderFlag>) -> &mut Self {
        self.order_flags.extend(flags);
        self
    }

    pub fn flag(&mut self, flag: OrderFlag) -> &mut Self {
        self.order_flags.insert(flag);
        self
    }

    pub fn build(&self) -> OrderBase {
        OrderBase {
            type_: self.type_.clone(),
            original_amount: self.original_amount,
            cumulative_amount: self.cumulative_amount,
            instrument: self.instrument.clone(),
            id: self.id.clone(),
            user_reference: self.user_reference.clone(),
            timestamp: self.timestamp,
            average_price: self.average_price,
            status: self.status,
            fee: self.fee,
            leverage: self.leverage.clone(),
            order_flags: self.order_flags.clone(),
        }
    }
}
