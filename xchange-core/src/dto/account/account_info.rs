use crate::dto::account::WalletFeature;
use crate::dto::account::open_position::OpenPosition;
use crate::dto::account::wallet::Wallet;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct AccountInfo {
    /// The name on the account
    pub username: Option<String>,

    /// The current fee this account must pay as a fraction of the value of each trade.
    /// None if there is no such fee.
    pub trading_fee: Option<Decimal>,

    /// The wallets owned by this account, keyed by wallet id
    pub wallets: HashMap<String, Wallet>,

    /// The open positions owned by this account
    pub open_positions: Vec<OpenPosition>,

    /// The timestamp at which this account information was generated
    pub timestamp: Option<DateTime<Utc>>,
}

impl AccountInfo {
    ///   Constructs an {@link AccountInfo}.
    ///
    ///     @param username the user name.
    ///     @param tradingFee the trading fee.
    ///     @param wallets the user's wallets
    ///     @param timestamp the timestamp for the account snapshot.
    pub fn new(
        username: Option<impl Into<String>>,
        trading_fee: Option<Decimal>,
        wallets: Vec<Wallet>,
        timestamp: Option<DateTime<Utc>>,
    ) -> Self {
        let username = username.map(|u| u.into());
        // 将 wallets 转成 HashMap，key 为 wallet.id
        let mut wallets_map = HashMap::new();
        for wallet in wallets.iter() {
            wallets_map.insert(wallet.id.clone(), wallet.clone());
        }

        Self {
            username: username.into(),
            trading_fee,
            wallets: wallets_map,
            open_positions: Vec::new(),
            timestamp,
        }
    }

    /// Constructs from wallets slice, username and trading fee optional
    pub fn from_wallets(wallets: &[Wallet]) -> Self {
        Self::new(None::<String>, None, wallets.to_vec(), None)
    }

    pub fn from_wallets_with_username(username: impl Into<String>, wallets: &[Wallet]) -> Self {
        Self::new(Some(username), None, wallets.to_vec(), None)
    }

    pub fn from_wallets_with_timestamp(timestamp: DateTime<Utc>, wallets: &[Wallet]) -> Self {
        Self::new(None::<String>, None, wallets.to_vec(), Some(timestamp))
    }

    ///   Constructs an {@link AccountInfo}.
    ///
    ///     @param username the user name.
    ///     @param tradingFee the trading fee.
    ///     @param wallets the user's wallets
    pub fn from_username_trading_fee_wallets(
        username: Option<impl Into<String>>,
        trading_fee: Option<Decimal>,
        wallets: Vec<Wallet>,
    ) -> Self {
        Self::new(username, trading_fee, wallets, None)
    }

    ///   Constructs an {@link AccountInfo}.
    ///
    ///    @param username the user name.
    ///    @param tradingFee the trading fee.
    ///    @param wallets the user's wallets
    ///    @param openPositions the users's open positions
    ///    @param timestamp the timestamp for the account snapshot.
    pub fn new_with_check(
        username: Option<impl Into<String>>,
        trading_fee: Option<Decimal>,
        wallets: Vec<Wallet>,
        open_positions: Vec<OpenPosition>,
        timestamp: Option<DateTime<Utc>>,
    ) -> Result<Self, String> {
        let username = username.map_or_else(|| "".to_string(), |u| u.into());

        // 构造 wallets HashMap 并检查重复
        let mut wallets_map = HashMap::new();
        for wallet in &wallets {
            if wallets_map.contains_key(&wallet.id) {
                return Err(format!(
                    "Duplicate wallets passed to AccountInfo: {}",
                    wallet.id
                ));
            }
            wallets_map.insert(wallet.id.clone(), wallet.clone());
        }

        Ok(Self {
            username: username.into(),
            trading_fee,
            wallets: wallets_map,
            open_positions,
            timestamp,
        })
    }

    /// Gets all wallets in this account
    pub fn wallets(&self) -> &HashMap<String, Wallet> {
        &self.wallets
    }

    /// Gets the wallet for accounts which don't use multiple wallets with ids
    pub fn wallet(&self) -> Result<&Wallet, String> {
        if self.wallets.len() != 1 {
            return Err(format!("{} wallets in account", self.wallets.len()));
        }
        Ok(self.wallets.values().next().unwrap())
    }

    /// Gets the wallet with a specific id
    pub fn wallet_by_id(&self, id: &str) -> Option<&Wallet> {
        self.wallets.get(id)
    }

    ///   Get wallet with given feature
    ///
    ///    @return null if no wallet on given exchange supports this feature
    ///    @throws UnsupportedOperationException if there are more then one wallets supporting the given
    ///        feature
    pub fn wallet_by_feature(&self, feature: WalletFeature) -> Result<Option<&Wallet>, String> {
        let wallets_with_feature: Vec<&Wallet> = self
            .wallets
            .values()
            .filter(|wallet| wallet.features.contains(&feature))
            .collect();

        match wallets_with_feature.len() {
            0 => Ok(None),
            1 => Ok(Some(wallets_with_feature[0])),
            _ => Err("More than one wallet offers this feature.".to_string()),
        }
    }

    /// Get the username of the account.
    pub fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    /// Returns the current trading fee, may be None if not provided.
    pub fn trading_fee(&self) -> Option<Decimal> {
        self.trading_fee
    }

    /// Returns the timestamp at which this account information was generated.
    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        self.timestamp
    }

    /// Returns all open positions.
    pub fn open_positions(&self) -> &Vec<OpenPosition> {
        &self.open_positions
    }
}

impl fmt::Display for AccountInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AccountInfo [username={:?}, tradingFee={:?}, wallets={:?}]",
            self.username, self.trading_fee, self.wallets
        )
    }
}
