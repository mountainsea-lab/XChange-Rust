use crate::dto::account::open_positions::OpenPositions;
use crate::dto::order::Order;
use crate::dto::trade::limit_order::LimitOrder;
use crate::dto::trade::market_order::MarketOrder;
use crate::dto::trade::open_orders::OpenOrders;
use crate::dto::trade::stop_order::StopOrder;
use crate::dto::trade::user_trades::UserTrades;
use crate::error::exchange_error::{
    ExchangeError, NotAvailableFromExchangeError, NotYetImplementedForExchangeError,
};
use crate::service::BaseService;
use crate::service::trade::params::orders::default_query_order_param::DefaultQueryOrderParam;
use crate::service::trade::params::orders::{OpenOrdersParams, OrderQueryParams};
use crate::service::trade::params::{CancelAllOrders, CancelOrderParams, TradeHistoryParams};
use std::collections::HashSet;

/// TradeService trait
pub trait TradeService: BaseService + Send + Sync {
    // ------------------ 核心交易方法 ------------------
    fn open_orders(&self) -> Result<OpenOrders, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("open_orders").into())
    }

    fn open_orders_with_params(
        &self,
        _params: &dyn OpenOrdersParams,
    ) -> Result<OpenOrders, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("open_orders_with_params").into())
    }

    fn open_positions(&self) -> Result<OpenPositions, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("open_positions").into())
    }

    fn place_market_order(&self, _order: &MarketOrder) -> Result<String, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("place_market_order").into())
    }

    fn place_limit_order(&self, _order: &LimitOrder) -> Result<String, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("place_limit_order").into())
    }

    fn place_stop_order(&self, _order: &StopOrder) -> Result<String, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("place_stop_order").into())
    }

    fn change_order(&self, order: &LimitOrder) -> Result<String, ExchangeError> {
        self.cancel_order_by_id(&order.order_base.id)?;
        self.place_limit_order(order)
    }

    fn cancel_order_by_id(&self, _order_id: &str) -> Result<bool, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("cancel_order_by_id").into())
    }

    fn cancel_order(&self, _params: &dyn CancelOrderParams) -> Result<bool, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("cancel_order").into())
    }

    fn cancel_all_orders(
        &self,
        _params: &dyn CancelAllOrders,
    ) -> Result<HashSet<String>, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("cancel_all_orders").into())
    }

    fn get_trade_history(
        &self,
        _params: &dyn TradeHistoryParams,
    ) -> Result<UserTrades, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("get_trade_history").into())
    }

    fn create_trade_history_params(&self) -> Result<Box<dyn TradeHistoryParams>, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("create_trade_history_params").into())
    }

    fn create_open_orders_params(&self) -> Result<Box<dyn OpenOrdersParams>, ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("create_open_orders_params").into())
    }

    fn verify_limit_order(&self, _order: &LimitOrder) -> Result<(), ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("verify_limit_order").into())
    }

    fn verify_market_order(&self, _order: &MarketOrder) -> Result<(), ExchangeError> {
        Err(NotYetImplementedForExchangeError::with_message("verify_market_order").into())
    }

    fn order_by_ids(&self, _order_ids: &[&str]) -> Result<Vec<Order>, ExchangeError> {
        Err(NotAvailableFromExchangeError::with_message("order_by_ids").into())
    }

    fn order_by_query(
        &self,
        _order_query: &[Box<dyn OrderQueryParams>],
    ) -> Result<Vec<Order>, ExchangeError> {
        Err(NotAvailableFromExchangeError::with_message("order_by_query").into())
    }
}

// ------------------ 静态辅助方法 ------------------

/// 将 order_id 列表转换为 OrderQueryParams 对象 Vec
fn to_order_query_params(order_ids: &[&str]) -> Vec<Box<dyn OrderQueryParams>> {
    order_ids
        .iter()
        .map(|id| {
            Box::new(DefaultQueryOrderParam {
                order_id: id.to_string(),
            }) as Box<dyn OrderQueryParams>
        })
        .collect()
}

/// 将 OrderQueryParams 对象转换为 order_id 列表
fn to_order_ids(order_query: &[Box<dyn OrderQueryParams>]) -> Vec<String> {
    order_query
        .iter()
        .map(|param| param.order_id().to_string())
        .collect()
}
