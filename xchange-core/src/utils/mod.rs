pub mod auth_utils;
pub mod time_nonce;

use crate::error::exchange_error::ExchangeError;
use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;
use serde::de::{self, Deserializer};
use serde_json::Value;
use std::any::Any;
use std::sync::Arc;

/// 辅助函数：尝试 Display，否则 Debug
pub fn display_value<T: std::fmt::Debug + 'static>(value: &T) -> String {
    if let Some(vec) = (value as &dyn std::any::Any).downcast_ref::<Vec<String>>() {
        vec.join(", ")
    } else {
        format!("{:?}", value)
    }
}

/// 反序列化 Binance serverTime（数字或字符串毫秒）为 `DateTime<Utc>`
pub fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    // 先把 JSON 反序列化成 Value
    let value = Value::deserialize(deserializer)?;

    // 根据类型处理
    let ts_millis = match value {
        Value::Number(num) => num
            .as_i64()
            .ok_or_else(|| de::Error::custom("Invalid number for timestamp"))?,
        Value::String(s) => s
            .parse::<i64>()
            .map_err(|_| de::Error::custom("Invalid string for timestamp"))?,
        _ => return Err(de::Error::custom("Unexpected type for timestamp")),
    };

    // 转换为 DateTime<Utc>
    Utc.timestamp_millis_opt(ts_millis)
        .single()
        .ok_or_else(|| de::Error::custom(format!("Invalid timestamp: {}", ts_millis)))
}

// 泛型获取 Exchange 内具体服务
// pub fn get_concrete_service<T: 'static>(
//     service: Arc<dyn Any + Send + Sync>,
// ) -> Result<Arc<T>, ExchangeError> {
//     service
//         .as_any()
//         .downcast_ref::<T>()
//         .map(|s| Arc::clone(s)) // 直接 clone Arc
//         .ok_or_else(|| {
//             ExchangeError::Message(format!(
//                 "Failed to downcast service to {}",
//                 std::any::type_name::<T>()
//             ))
//         })
// }
