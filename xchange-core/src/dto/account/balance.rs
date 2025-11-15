use crate::currency::currency::Currency;
use chrono::{DateTime, Utc};
use log::warn;
use rust_decimal::Decimal;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

///
///  DTO representing a balance in a currency
///
///   <p>This is simply defined by an amount of money in a given currency, contained in the cash
///   object.
///
///   <p>This class is immutable.
///
pub struct Balance {
    pub currency: Currency,
    // Invariant:
    // total = available + frozen - borrowed + loaned + withdrawing + depositing;
    pub total: Option<Decimal>,
    pub available: Option<Decimal>,
    pub frozen: Option<Decimal>,
    pub loaned: Decimal,
    pub borrowed: Decimal,
    pub withdrawing: Decimal,
    pub depositing: Decimal,

    pub timestamp: Option<DateTime<Utc>>,
}

impl Balance {
    ///  Constructs a balance, the {@link #available} will be the same as the <code>total</code>, and
    ///   the {@link #frozen} is zero. The <code>borrowed</code> and <code>loaned</code> will be zero.
    ///
    ///   @param currency The underlying currency
    ///   @param total The total
    pub fn new(currency: Currency, total: Decimal) -> Self {
        Balance {
            currency,
            total: Some(total),
            available: Some(total),
            frozen: Some(Decimal::ZERO),
            borrowed: Decimal::ZERO,
            loaned: Decimal::ZERO,
            withdrawing: Decimal::ZERO,
            depositing: Decimal::ZERO,
            timestamp: None,
        }
    }

    ///
    ///     Constructs a balance, the {@link #frozen} will be assigned as <code>total</code> - <code>
    ///     available</code>. The <code>borrowed</code> and <code>loaned</code> will be zero.
    ///
    ///     @param currency the underlying currency of this balance.
    ///     @param total the total amount of the <code>currency</code> in this balance.
    ///     @param available the amount of the <code>currency</code> in this balance that is available to
    ///         trade.
    ///
    pub fn new_with_available(currency: Currency, total: Decimal, available: Decimal) -> Self {
        Balance {
            currency,
            total: Some(total),
            available: Some(available),
            frozen: Some(total - available),
            borrowed: Decimal::ZERO,
            loaned: Decimal::ZERO,
            withdrawing: Decimal::ZERO,
            depositing: Decimal::ZERO,
            timestamp: None,
        }
    }

    ///     Constructs a balance. The <code>borrowed</code> and <code>loaned</code> will be zero.
    ///
    ///     @param currency the underlying currency of this balance.
    ///     @param total the total amount of the <code>currency</code> in this balance, including the
    ///         <code>available</code> and <code>frozen</code>.
    ///     @param available the amount of the <code>currency</code> in this balance that is available to
    ///         trade.
    ///     @param frozen the frozen amount of the <code>currency</code> in this balance that is locked in
    ///         trading.
    pub fn new_with_frozen(
        currency: Currency,
        total: Decimal,
        available: Decimal,
        frozen: Decimal,
    ) -> Self {
        Balance {
            currency,
            total: Some(total),
            available: Some(available),
            frozen: Some(frozen),
            borrowed: Decimal::ZERO,
            loaned: Decimal::ZERO,
            withdrawing: Decimal::ZERO,
            depositing: Decimal::ZERO,
            timestamp: None,
        }
    }

    /// Constructs a balance.
    ///
    ///     @param currency the underlying currency of this balance.
    ///     @param total the total amount of the <code>currency</code> in this balance, equal to <code>
    ///         available + frozen - borrowed + loaned</code>.
    ///     @param available the amount of the <code>currency</code> in this balance that is available to
    ///         trade, including the <code>borrowed</code>.
    ///     @param frozen the frozen amount of the <code>currency</code> in this balance that is locked in
    ///         trading.
    ///     @param borrowed the borrowed amount of the available <code>currency</code> in this balance that
    ///         must be repaid.
    ///     @param loaned the loaned amount of the total <code>currency</code> in this balance that will be
    ///         returned.
    ///     @param withdrawing the amount of the <code>currency</code> in this balance that is scheduled
    ///        for withdrawal.
    ///     @param depositing the amount of the <code>currency</code> in this balance that is being
    ///         deposited but not available yet.
    ///     @param timestamp Time the balance was valid on the exchange server
    pub fn new_full(
        currency: Currency,
        total: Decimal,
        available: Decimal,
        frozen: Decimal,
        borrowed: Decimal,
        loaned: Decimal,
        withdrawing: Decimal,
        depositing: Decimal,
        timestamp: Option<DateTime<Utc>>,
    ) -> Self {
        // Invariant check (total == available + frozen - borrowed + loaned + withdrawing + depositing)
        let sum = available + frozen - borrowed + loaned + withdrawing + depositing;

        if total != sum {
            warn!(
                "{} = total != available + frozen - borrowed + loaned + withdrawing + depositing = {}",
                total, sum
            );
        }

        Balance {
            currency,
            total: Some(total),
            available: Some(available),
            frozen: Some(frozen),
            borrowed,
            loaned,
            withdrawing,
            depositing,
            timestamp,
        }
    }

    ///     Constructs a balance.
    ///
    ///     @param currency the underlying currency of this balance.
    ///     @param total the total amount of the <code>currency</code> in this balance, equal to <code>
    ///         available + frozen - borrowed + loaned</code>.
    ///     @param available the amount of the <code>currency</code> in this balance that is available to
    ///         trade, including the <code>borrowed</code>.
    ///     @param frozen the frozen amount of the <code>currency</code> in this balance that is locked in
    ///         trading.
    ///     @param borrowed the borrowed amount of the available <code>currency</code> in this balance that
    ///         must be repaid.
    ///     @param loaned the loaned amount of the total <code>currency</code> in this balance that will be
    ///         returned.
    ///     @param withdrawing the amount of the <code>currency</code> in this balance that is scheduled
    ///         for withdrawal.
    ///     @param depositing the amount of the <code>currency</code> in this balance that is being
    ///         deposited but not available yet.
    pub fn new_no_timestamp(
        currency: Currency,
        total: Decimal,
        available: Decimal,
        frozen: Decimal,
        borrowed: Decimal,
        loaned: Decimal,
        withdrawing: Decimal,
        depositing: Decimal,
    ) -> Self {
        // Invariant: total == available + frozen - borrowed + loaned + withdrawing + depositing
        let sum = available + frozen - borrowed + loaned + withdrawing + depositing;

        if total != sum {
            warn!(
                "{} = total != available + frozen - borrowed + loaned + withdrawing + depositing = {}",
                total, sum
            );
        }

        Balance {
            currency,
            total: Some(total),
            available: Some(available),
            frozen: Some(frozen),
            borrowed,
            loaned,
            withdrawing,
            depositing,
            timestamp: None, // fully matches Java
        }
    }

    ///  Returns a zero balance.
    ///
    ///  @param currency the balance currency.
    ///  @return a zero balance.
    pub fn zero(currency: Currency) -> Self {
        Balance {
            currency,
            total: Some(Decimal::ZERO),
            available: Some(Decimal::ZERO),
            frozen: Some(Decimal::ZERO),
            borrowed: Decimal::ZERO,
            loaned: Decimal::ZERO,
            withdrawing: Decimal::ZERO,
            depositing: Decimal::ZERO,
            timestamp: None, // same as Java's null
        }
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    ///  Returns the total amount of the <code>currency</code> in this balance.
    ///
    ///  @return the total amount.
    pub fn total(&self) -> Decimal {
        let available = self.available.unwrap_or(Decimal::ZERO);
        let frozen = self.frozen.unwrap_or(Decimal::ZERO);
        self.total.unwrap_or(
            available + frozen - self.borrowed + self.loaned + self.withdrawing + self.depositing,
        )
    }

    /// Returns the amount of the <code>currency</code> in this balance that is available to trade.
    ///
    ///  @return the amount that is available to trade.
    pub fn available(&self) -> Decimal {
        let frozen = self.frozen.unwrap_or(Decimal::ZERO);
        match self.available {
            Some(a) => a,
            None => {
                self.total() - frozen - self.loaned + self.borrowed
                    - self.withdrawing
                    - self.depositing
            }
        }
    }

    /// Returns the amount of the <code>currency</code> in this balance that may be withdrawn. Equal to
    ///    <code>available - borrowed</code>.
    ///
    ///    @return the amount that is available to withdraw.
    pub fn available_for_withdrawal(&self) -> Decimal {
        self.available() - self.borrowed()
    }

    /// Returns the frozen amount of the <code>currency</code> in this balance that is locked in
    ///    trading.
    ///
    ///    @return the amount that is locked in open orders.
    pub fn frozen(&self) -> Decimal {
        match self.frozen {
            Some(f) => f,
            None => self.total() - self.available(),
        }
    }

    /// Returns the borrowed amount of the available <code>currency</code> in this balance that must be
    ///    repaid.
    ///
    ///    @return the amount that must be repaid.
    pub fn borrowed(&self) -> Decimal {
        self.borrowed
    }

    /// Returns the loaned amount of the total <code>currency</code> in this balance that will be
    ///     returned.
    ///
    ///     @return that amount that is loaned out.
    pub fn loaned(&self) -> Decimal {
        self.loaned
    }

    /// Returns the amount of the <code>currency</code> in this balance that is locked in withdrawal
    ///
    ///   @return the amount in withdrawal.
    pub fn withdrawing(&self) -> Decimal {
        self.withdrawing
    }

    /// Returns the amount of the <code>currency</code> in this balance that is locked in deposit
    ///
    ///    @return the amount in deposit.
    pub fn depositing(&self) -> Decimal {
        self.depositing
    }

    /// Returns the time the balance was valid on the exchange server
    ///
    ///    @return the timestamp.
    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        self.timestamp
    }
}

impl fmt::Display for Balance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Balance{{currency={}, total={}, available={}, frozen={}, loaned={}, borrowed={}, withdrawing={}, depositing={}, timestamp={:?}}}",
            self.currency,
            self.total(),
            self.available(),
            self.frozen(),
            self.loaned,
            self.borrowed,
            self.withdrawing,
            self.depositing,
            self.timestamp
        )
    }
}

impl Hash for Balance {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.total.hash(state);
        self.currency.hash(state);
        self.available.hash(state);
        self.frozen.hash(state);
        self.borrowed.hash(state);
        self.loaned.hash(state);
        self.withdrawing.hash(state);
        self.depositing.hash(state);
    }
}

impl PartialEq for Balance {
    fn eq(&self, other: &Self) -> bool {
        self.total == other.total
            && self.available == other.available
            && self.frozen == other.frozen
            && self.currency == other.currency
            && self.borrowed == other.borrowed
            && self.loaned == other.loaned
            && self.withdrawing == other.withdrawing
            && self.depositing == other.depositing
    }
}

impl Eq for Balance {}

impl PartialOrd for Balance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Balance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.currency
            .cmp(&other.currency)
            .then_with(|| self.total.cmp(&other.total))
            .then_with(|| self.available.cmp(&other.available))
            .then_with(|| self.frozen.cmp(&other.frozen))
            .then_with(|| self.borrowed.cmp(&other.borrowed))
            .then_with(|| self.loaned.cmp(&other.loaned))
            .then_with(|| self.withdrawing.cmp(&other.withdrawing))
            .then_with(|| self.depositing.cmp(&other.depositing))
    }
}

#[derive(Debug, Clone)]
pub struct BalanceBuilder {
    currency: Option<Currency>,
    total: Option<Decimal>,
    available: Option<Decimal>,
    frozen: Option<Decimal>,
    borrowed: Decimal,
    loaned: Decimal,
    withdrawing: Decimal,
    depositing: Decimal,
    timestamp: Option<DateTime<Utc>>,
}

impl BalanceBuilder {
    pub fn new() -> Self {
        Self {
            currency: None,
            total: None,
            available: None,
            frozen: None,
            borrowed: Decimal::ZERO,
            loaned: Decimal::ZERO,
            withdrawing: Decimal::ZERO,
            depositing: Decimal::ZERO,
            timestamp: None,
        }
    }

    pub fn from(balance: &Balance) -> Self {
        Self {
            currency: Some(balance.currency.clone()),
            total: balance.total,
            available: balance.available,
            frozen: balance.frozen,
            borrowed: balance.borrowed,
            loaned: balance.loaned,
            withdrawing: balance.withdrawing,
            depositing: balance.depositing,
            timestamp: balance.timestamp.clone(),
        }
    }

    pub fn currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn total(mut self, total: Decimal) -> Self {
        self.total = Some(total);
        self
    }

    pub fn available(mut self, available: Decimal) -> Self {
        self.available = Some(available);
        self
    }

    pub fn frozen(mut self, frozen: Decimal) -> Self {
        self.frozen = Some(frozen);
        self
    }

    pub fn borrowed(mut self, borrowed: Decimal) -> Self {
        self.borrowed = borrowed;
        self
    }

    pub fn loaned(mut self, loaned: Decimal) -> Self {
        self.loaned = loaned;
        self
    }

    pub fn withdrawing(mut self, withdrawing: Decimal) -> Self {
        self.withdrawing = withdrawing;
        self
    }

    pub fn depositing(mut self, depositing: Decimal) -> Self {
        self.depositing = depositing;
        self
    }

    pub fn timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn build(self) -> Result<Balance, String> {
        let currency = self
            .currency
            .ok_or_else(|| "currency is required".to_string())?;

        let total = self.total.ok_or_else(|| "total is required".to_string())?;

        let frozen = match self.frozen {
            Some(f) => f,
            None => {
                if let (Some(total), Some(available)) = (self.total, self.available) {
                    total - available
                } else {
                    Decimal::ZERO
                }
            }
        };

        Ok(Balance {
            currency,
            total: Some(total),
            available: self.available,
            frozen: Some(frozen),
            borrowed: self.borrowed,
            loaned: self.loaned,
            withdrawing: self.withdrawing,
            depositing: self.depositing,
            timestamp: self.timestamp,
        })
    }
}
