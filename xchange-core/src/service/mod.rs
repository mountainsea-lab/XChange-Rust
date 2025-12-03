use crate::dto::meta::exchange_metadata::ExchangeMetaData;
use crate::dto::trade::limit_order::LimitOrder;
use crate::dto::trade::market_order::MarketOrder;
use crate::exchange::Exchange;
use async_trait::async_trait;
use std::any::Any;
use std::sync::Arc;

pub mod account;
pub mod marketdata;
pub mod trade;

// #[async_trait]
pub trait BaseService: Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

pub trait ExchangeService<E: Exchange> {
    fn exchange(&self) -> &E;

    fn verify_limit_order(&self, order: &LimitOrder) -> Result<(), String> {
        // let meta = self.exchange().exchange_meta_data();
        // self.verify_order(order, &meta.clone())?;
        // let price = order.limit_price().ok_or("Missing limit price")?;
        // if price.scale() > meta.price_scale(order.currency_pair()) {
        //     return Err(format!("Unsupported price scale {}", price.scale()));
        // }
        Ok(())
    }
    // <O: BaseOrder>
    fn verify_order(
        &self,
        order: &MarketOrder,
        meta_data: &ExchangeMetaData,
    ) -> Result<(), String> {
        // let meta = meta_data.get_instrument(order.currency_pair())
        //     .ok_or("Invalid CurrencyPair")?;
        // let amount = order.original_amount().ok_or("Missing originalAmount")?;
        // let amount = amount.strip_trailing_zeros();
        // if let Some(min_amount) = meta.minimum_amount() {
        //     if amount.scale() > min_amount.scale() {
        //         return Err(format!("Unsupported amount scale {}", amount.scale()));
        //     } else if amount < min_amount {
        //         return Err("Order amount less than minimum".to_string());
        //     }
        // }
        Ok(())
    }
}
