use crate::currency::currency::Currency;
use crate::dto::account::WalletFeature;
use crate::dto::account::balance::Balance;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};

///  DTO representing a wallet
///
///  <p>A wallet has a set of current balances in various currencies held on the exchange.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

    /// @return The wallet id
    pub fn id(&self) -> &str {
        &self.id
    }

    /// @return A descriptive name for the wallet
    pub fn name(&self) -> &str {
        &self.name
    }

    /// @return The available colletion of balances
    pub fn balances(&self) -> &Vec<Balance> {
        &self.balance_collection
    }

    /// @return The available balances (amount and currency)
    pub fn balances_map(&self) -> &HashMap<Currency, Balance> {
        &self.balances
    }

    /// @return All wallet operation features
    pub fn features(&self) -> &HashSet<WalletFeature> {
        &self.features
    }

    /// @return Max leverage of wallet
    pub fn max_leverage(&self) -> Option<Decimal> {
        self.max_leverage
    }

    /// @return current leverage of wallet
    pub fn current_leverage(&self) -> Option<Decimal> {
        self.current_leverage
    }

    ///    Returns the balance for the specified currency.
    ///
    ///    @param currency a {@link Currency}.
    ///    @return the balance of the specified currency, or a zero balance if currency not present
    pub fn get_balance(&self, currency: &Currency) -> Balance {
        self.balances
            .get(currency)
            .cloned()
            .unwrap_or_else(|| Balance::zero(currency.clone()))
    }
}

impl Hash for Wallet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);

        // 将 HashMap 转成迭代器，再对每个键值对调用 hash
        let mut items: Vec<_> = self.balances.iter().collect();
        items.sort_by(|a, b| a.0.cmp(b.0)); // 按 Currency 排序保证顺序一致
        for (k, v) in items {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl PartialEq for Wallet {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name && self.balances == other.balances
    }
}

impl Eq for Wallet {}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Wallet{{balances={:?}, id='{}', name='{}', walletFeatures={:?}, maxLeverage={:?}, currentLeverage={:?}}}",
            self.balance_collection,
            self.id,
            self.name,
            self.features,
            self.max_leverage,
            self.current_leverage
        )
    }
}

pub struct WalletBuilder {
    balances: Vec<Balance>,
    id: Option<String>,
    name: Option<String>,
    features: HashSet<WalletFeature>,
    max_leverage: Option<Decimal>,
    current_leverage: Option<Decimal>,
}

impl WalletBuilder {
    /// 创建一个新的 Builder，默认 features 为 TRADING 和 FUNDING
    pub fn new() -> Self {
        let mut features = HashSet::new();
        features.insert(WalletFeature::Trading);
        features.insert(WalletFeature::Funding);

        Self {
            balances: Vec::new(),
            id: None,
            name: None,
            features,
            max_leverage: Some(Decimal::ZERO),
            current_leverage: Some(Decimal::ZERO),
        }
    }

    pub fn from(balances: Vec<Balance>) -> Self {
        let mut builder = Self::new();
        builder.balances = balances;
        builder
    }

    pub fn balances(mut self, balances: Vec<Balance>) -> Self {
        self.balances = balances;
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn features(mut self, features: HashSet<WalletFeature>) -> Self {
        self.features = features;
        self
    }

    pub fn max_leverage(mut self, max: Decimal) -> Self {
        self.max_leverage = Some(max);
        self
    }

    pub fn current_leverage(mut self, current: Decimal) -> Self {
        self.current_leverage = Some(current);
        self
    }

    /// 构建 Wallet
    pub fn build(self) -> Result<Wallet, String> {
        let id = self.id.ok_or("Wallet id is required")?;
        let name = self.name.unwrap_or_else(|| id.clone());

        Wallet::new(
            id,
            Some(name),
            self.balances,
            self.features,
            self.max_leverage,
            self.current_leverage,
        )
    }
}
