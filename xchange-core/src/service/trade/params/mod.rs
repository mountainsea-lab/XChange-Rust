pub mod orders;

pub trait CancelOrderParams {}

/// Marker trait for all parameter types used in `TradeService::get_trade_history`
pub trait TradeHistoryParams {}

/// Marker trait for canceling all orders, extending `CancelOrderParams`
pub trait CancelAllOrders: CancelOrderParams {}
