use crate::binance_exchange::BinanceExchange;
use crate::dto::BinanceError;
use crate::dto::account::binance_currency_info::BinanceCurrencyInfo;
use crate::service::binance_base_service::BinanceBaseService;
use parking_lot::RwLock;
use std::sync::Arc;

pub struct BinanceAccountServiceRaw {
    pub base: Arc<BinanceBaseService>,
    // 对应 currencyInfos 缓存和锁
    currency_infos: RwLock<Option<Vec<BinanceCurrencyInfo>>>,
}

impl BinanceAccountServiceRaw {
    pub fn new(exchange: Arc<BinanceExchange>) -> Result<Self, BinanceError> {
        let base = BinanceBaseService::new(exchange.clone())
            .map_err(|e| BinanceError::ServiceNotInitialized(e.to_string()))?;

        Ok(Self {
            base: Arc::new(base),
            currency_infos: RwLock::new(None),
        })
    }
}
// impl BinanceAccountServiceRaw {
//     pub fn new(base: Arc<BinanceBaseService>) -> Self {
//         Self {
//             base,
//             currency_infos: RwLock::new(None),
//         }
//     }
//
//     // --------------------------
//     // Account / Currency
//     // --------------------------
//     pub async fn account(&self) -> BinanceAccountInformation {
//         todo!("实现 API 调用 + decorateApiCall + retry + rate limiter")
//     }
//
//     pub async fn currency_infos(&self) -> Vec<BinanceCurrencyInfo> {
//         todo!()
//     }
//
//     pub async fn futures_account(&self, _use_v3: bool) -> BinanceFutureAccountInformation {
//         todo!()
//     }
//
//     // --------------------------
//     // Withdraw / Deposit
//     // --------------------------
//     pub async fn withdraw(
//         &self,
//         _coin: &str,
//         _address: &str,
//         _address_tag: Option<&str>,
//         _amount: f64,
//         _name: &str,
//         _network: Option<&str>,
//     ) -> WithdrawResponse {
//         todo!()
//     }
//
//     pub async fn request_deposit_address(
//         &self,
//         _currency: &Currency,
//         _network: Option<&str>,
//     ) -> DepositAddress {
//         todo!()
//     }
//
//     pub async fn request_asset_detail(&self) -> std::collections::HashMap<String, AssetDetail> {
//         todo!()
//     }
//
//     // --------------------------
//     // Deposit / Withdraw history
//     // --------------------------
//     pub async fn deposit_history(
//         &self,
//         _asset: &str,
//         _start_time: Option<u64>,
//         _end_time: Option<u64>,
//     ) -> Vec<BinanceDeposit> {
//         todo!()
//     }
//
//     pub async fn withdraw_history(
//         &self,
//         _asset: &str,
//         _start_time: Option<u64>,
//         _end_time: Option<u64>,
//     ) -> Vec<BinanceWithdraw> {
//         todo!()
//     }
//
//     // --------------------------
//     // Asset dividend
//     // --------------------------
//     pub async fn get_asset_dividend(
//         &self,
//         _asset: Option<&str>,
//         _start_time: Option<u64>,
//         _end_time: Option<u64>,
//     ) -> Vec<AssetDividend> {
//         todo!()
//     }
//
//     // --------------------------
//     // Transfer history
//     // --------------------------
//     pub async fn get_transfer_history(
//         &self,
//         _from_email: Option<&str>,
//         _start_time: Option<u64>,
//         _end_time: Option<u64>,
//         _page: Option<u32>,
//         _limit: Option<u32>,
//     ) -> Vec<TransferHistory> {
//         todo!()
//     }
//
//     pub async fn get_sub_user_history(
//         &self,
//         _asset: &str,
//         _type_: Option<u32>,
//         _start_time: Option<u64>,
//         _end_time: Option<u64>,
//         _limit: Option<u32>,
//     ) -> Vec<TransferSubUserHistory> {
//         todo!()
//     }
//
//     // --------------------------
//     // Futures / Margin / Leverage
//     // --------------------------
//     pub async fn set_margin_type(&self, _instrument: &Instrument, _margin_type: MarginType)
//                                  -> BinanceChangeStatus {
//         todo!()
//     }
//
//     pub async fn set_dual_side_position(&self, _dual_side: bool) -> BinanceChangeStatus {
//         todo!()
//     }
//
//     pub async fn set_leverage_raw(&self, _instrument: &Instrument, _leverage: u32)
//                                   -> BinanceSetLeverage {
//         todo!()
//     }
//
//     // --------------------------
//     // Currency info cache
//     // --------------------------
//     pub fn get_currency_info_cached(&self) -> Vec<BinanceCurrencyInfo> {
//         let mut lock = self.currency_infos.write().unwrap();
//         if lock.is_none() {
//             // todo: 调用 currency_infos() 填充缓存
//             *lock = Some(vec![]);
//         }
//         lock.clone().unwrap()
//     }
//
//     pub fn get_currency_info(&self, currency: &Currency) -> Option<BinanceCurrencyInfo> {
//         self.get_currency_info_cached()
//             .into_iter()
//             .find(|info| &info.currency == currency)
//     }
// }
