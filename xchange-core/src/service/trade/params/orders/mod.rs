use crate::dto::order::Order;
use crate::dto::trade::limit_order::LimitOrder;

pub mod default_query_order_param;

/// Root trait for all parameter types used in `TradeService::open_orders_with_params`
pub trait OpenOrdersParams {
    /// Checks if a limit order is suitable for open orders params.
    fn accept_limit_order(&self, order: &LimitOrder) -> bool;

    /// Checks if any order (limit or stop) is suitable.
    /// Default implementation converts to `LimitOrder` for filtering.
    fn accept_order(&self, order: &Order) -> bool {
        // Default: convert Order -> LimitOrder for filtering
        let limit_order = order.as_limit_order();
        if let Some(limit_order) = limit_order {
            self.accept_limit_order(limit_order)
        } else {
            false
        }
    }
}

/// Trait representing query parameters for fetching orders from an exchange.
/// Exchanges can implement their own struct if querying an order requires
/// additional information beyond the order ID.
pub trait OrderQueryParams {
    /// Get the order ID
    fn order_id(&self) -> &str;

    /// Set the order ID
    fn set_order_id(&mut self, order_id: String);
}
