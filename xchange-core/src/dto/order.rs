use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    BID,      // Buying order (the trader is providing the counter currency)
    ASK,      // Selling order (the trader is providing the base currency)
    EXIT_ASK, // Close a short position in crypto derivatives
    EXIT_BID, // Close a long position in crypto derivatives
}

impl OrderType {
    // Method to get the opposite of the order type
    pub fn get_opposite(self) -> Option<OrderType> {
        match self {
            OrderType::BID => Some(OrderType::ASK),
            OrderType::ASK => Some(OrderType::BID),
            OrderType::EXIT_ASK => Some(OrderType::EXIT_BID),
            OrderType::EXIT_BID => Some(OrderType::EXIT_ASK),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    PENDING_NEW,        // Initial order when instantiated
    NEW,                // Initial order when placed on the order book at exchange
    PARTIALLY_FILLED,   // Partially match against opposite order on order book at exchange
    FILLED,             // Fully match against opposite order on order book at exchange
    PENDING_CANCEL,     // Waiting to be removed from order book at exchange
    PARTIALLY_CANCELED, // Order was partially canceled at exchange
    CANCELED,           // Removed from order book at exchange
    PENDING_REPLACE,    // Waiting to be replaced by another order on order book at exchange
    REPLACED,           // Order has been replaced by another order on order book at exchange
    STOPPED,            // Order has been triggered at stop price
    REJECTED,           // Order has been rejected by exchange and not placed on order book
    EXPIRED,            // Order has expired and been removed from order book
    OPEN,               // Order is open and waiting to be filled
    CLOSED,             // Order has been either filled or cancelled
    UNKNOWN,            // The exchange returned a state not in the documentation
}

impl OrderStatus {
    // Method to check if the order status is final
    pub fn is_final(self) -> bool {
        matches!(
            self,
            OrderStatus::FILLED
                | OrderStatus::PARTIALLY_CANCELED
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
            OrderStatus::PENDING_NEW
                | OrderStatus::NEW
                | OrderStatus::PARTIALLY_FILLED
                | OrderStatus::OPEN
        )
    }
}

// Trait to represent the shared behavior of different orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Order {
    LimitOrder(LimitOrder),
    StopOrder(StopOrder),
    MarketOrder(MarketOrder),
}

impl Order {
    // Example method to retrieve the ID
    pub fn id(&self) -> &str {
        match self {
            Order::LimitOrder(order) => &order.id,
            Order::StopOrder(order) => &order.id,
            Order::MarketOrder(order) => &order.id,
        }
    }
}