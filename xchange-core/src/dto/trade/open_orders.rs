use crate::dto::order::Order;
use crate::dto::trade::limit_order::LimitOrder;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use std::fmt::Write; // For `write!` and `fmt::Error`

/// Represents open orders. These are orders that have been placed with the exchange but have not yet been matched to a counterparty.
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenOrders {
    /// The list of open limit orders.
    pub open_orders: Vec<LimitOrder>,
    /// The list of hidden orders, such as untriggered stop orders.
    pub hidden_orders: Vec<Order>,
}

impl OpenOrders {
    /// Constructor for creating a new `OpenOrders` instance with only open orders.
    ///
    /// # Parameters
    /// - `open_orders`: A list of open limit orders.
    ///
    /// # Returns
    /// A new instance of `OpenOrders`.
    pub fn new(open_orders: Vec<LimitOrder>, hidden_orders: Vec<Order>) -> Self {
        OpenOrders {
            open_orders,
            hidden_orders,
        }
    }

    /// Constructor for creating a new `OpenOrders` instance with both open and hidden orders.
    ///
    /// # Parameters
    /// - `open_orders`: A list of open limit orders.
    /// - `hidden_orders`: A list of hidden orders.
    ///
    /// # Returns
    /// A new instance of `OpenOrders`.
    pub fn new_with_hidden_orders(open_orders: Vec<LimitOrder>, hidden_orders: Vec<Order>) -> Self {
        OpenOrders {
            open_orders,
            hidden_orders,
        }
    }

    // Method to get the open orders (LimitOrders).
    pub fn get_open_orders(&self) -> &Vec<LimitOrder> {
        &self.open_orders
    }

    // Method to get all orders (both open and hidden).
    pub fn get_all_open_orders(&self) -> Vec<Order> {
        let mut all_open_orders = Vec::new();

        // Push the owned values (not references)
        for order in &self.open_orders {
            all_open_orders.push(Order::LimitOrder(order.clone()));
        }

        // Push hidden orders as well
        for order in &self.hidden_orders {
            all_open_orders.push(order.clone()); // Clone hidden orders
        }

        all_open_orders
    }

    // Method to get hidden orders.
    pub fn get_hidden_orders(&self) -> &Vec<Order> {
        &self.hidden_orders
    }
}

impl fmt::Display for OpenOrders {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        let _ = add_to_to_string(&mut output, &self.open_orders, "Open orders");
        let _ = add_to_to_string(&mut output, &self.hidden_orders, "Hidden orders");

        write!(f, "{}", output)
    }
}

fn add_to_to_string<T: fmt::Debug>(
    output: &mut String,
    orders: &[T],
    description: &str,
) -> fmt::Result {
    output.push_str(description);
    output.push_str(": ");

    if orders.is_empty() {
        output.push_str("None\n");
    } else {
        output.push_str("\n");
        for order in orders {
            // Handle the error from `write!` by returning it
            write!(output, "  [order={:?}]\n", order)?;
        }
    }

    Ok(()) // Return Ok if everything is successful
}
