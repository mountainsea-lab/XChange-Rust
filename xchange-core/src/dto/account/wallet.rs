use crate::currency::currency::Currency;
use crate::dto::account::WalletFeature;
use crate::dto::account::balance::Balance;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

///  DTO representing a wallet
///
///  <p>A wallet has a set of current balances in various currencies held on the exchange.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wallet {
    /// The keys represent the currency of the wallet
    pub balances: HashMap<Currency, Balance>,

    /// Collection of balances for deserialization
    pub balance_collection: Vec<Balance>,

    /// Unique identifier for this wallet
    pub id: String,

    /// Descriptive name for this wallet. Defaults to id
    pub name: String,

    /// Features supported by this wallet
    pub features: HashSet<WalletFeature>,

    /// Maximum leverage for margin trading supported by this wallet
    pub max_leverage: Option<Decimal>,

    /// Current leverage for margin trading done on this wallet
    pub current_leverage: Option<Decimal>,
}

impl Wallet {
    ///  Constructs a {@link Wallet}.
    ///
    ///  @param id the wallet id
    ///  @param name a descriptive name for the wallet
    ///  @param balances the balances, the currencies of the balances should not be duplicated.
    ///  @param features all the features that wallet supports
    ///   <p>maxLeverage and currentLeverage are BigDecimal.ZERO for the default constructor
    pub fn new(
        id: impl Into<String>,
        name: Option<impl Into<String>>,
        balances: Vec<Balance>,
        features: HashSet<WalletFeature>,
        max_leverage: Option<Decimal>,
        current_leverage: Option<Decimal>,
    ) -> Result<Self, String> {
        let id = id.into();
        let name = name.map_or_else(|| id.clone(), |n| n.into());

        let balances_clone = balances.clone();
        // 构造 balances Map，检查重复的 currency
        let mut balances_map = HashMap::with_capacity(balances.len());
        for balance in balances.into_iter() {
            // balance 是 Balance
            let currency = balance.currency.clone();
            if balances_map.insert(currency.clone(), balance).is_some() {
                return Err(format!(
                    "Duplicate balances in wallet for currency: {}",
                    currency
                ));
            }
        }

        Ok(Self {
            id,
            name,
            balances: balances_map,
            balance_collection: balances_clone, // 移动所有权，不再 clone
            features,
            max_leverage,
            current_leverage,
        })
    }
}
