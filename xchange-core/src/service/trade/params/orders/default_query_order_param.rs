use crate::service::trade::params::orders::OrderQueryParams;

/// 默认实现，仅包含 order_id 字段
#[derive(Debug, Clone, Default)]
pub struct DefaultQueryOrderParam {
    pub order_id: String,
}

impl DefaultQueryOrderParam {
    /// 创建一个空的 DefaultQueryOrderParam
    pub fn new() -> Self {
        Self {
            order_id: String::new(),
        }
    }

    /// 创建一个带 order_id 的 DefaultQueryOrderParam
    pub fn with_order_id(order_id: impl Into<String>) -> Self {
        Self {
            order_id: order_id.into(),
        }
    }
}

impl OrderQueryParams for DefaultQueryOrderParam {
    fn order_id(&self) -> &str {
        &self.order_id
    }

    fn set_order_id(&mut self, order_id: String) {
        self.order_id = order_id;
    }
}
