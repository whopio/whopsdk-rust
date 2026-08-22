pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountBalanceToken {
    /// Total amount held in native units, as a decimal string.
    #[serde(default)]
    pub balance: String,
    /// Balance split into available, pending, and reserve amounts, as native-unit decimal strings, with the days the pending amount is expected to settle. On-chain crypto is entirely available; good_funds and fiat cash can have pending or reserve portions.
    #[serde(default)]
    pub breakdown: AccountBalanceBreakdown,
    /// Holding icon URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// The holding's display name
    #[serde(default)]
    pub name: String,
    /// USD price per unit, or `null` when no exchange rate is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price_usd: Option<f64>,
    /// Holding display symbol, such as `USDT`, `cbBTC`, or `EUR`.
    #[serde(default)]
    pub symbol: String,
    /// Holding USD value, or `null` when no exchange rate is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_usd: Option<String>,
}

impl AccountBalanceToken {
    pub fn builder() -> AccountBalanceTokenBuilder {
        <AccountBalanceTokenBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountBalanceTokenBuilder {
    balance: Option<String>,
    breakdown: Option<AccountBalanceBreakdown>,
    icon_url: Option<String>,
    name: Option<String>,
    price_usd: Option<f64>,
    symbol: Option<String>,
    value_usd: Option<String>,
}

impl AccountBalanceTokenBuilder {
    pub fn balance(mut self, value: impl Into<String>) -> Self {
        self.balance = Some(value.into());
        self
    }

    pub fn breakdown(mut self, value: AccountBalanceBreakdown) -> Self {
        self.breakdown = Some(value);
        self
    }

    pub fn icon_url(mut self, value: impl Into<String>) -> Self {
        self.icon_url = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn price_usd(mut self, value: f64) -> Self {
        self.price_usd = Some(value);
        self
    }

    pub fn symbol(mut self, value: impl Into<String>) -> Self {
        self.symbol = Some(value.into());
        self
    }

    pub fn value_usd(mut self, value: impl Into<String>) -> Self {
        self.value_usd = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountBalanceToken`].
    /// This method will fail if any of the following fields are not set:
    /// - [`balance`](AccountBalanceTokenBuilder::balance)
    /// - [`breakdown`](AccountBalanceTokenBuilder::breakdown)
    /// - [`name`](AccountBalanceTokenBuilder::name)
    /// - [`symbol`](AccountBalanceTokenBuilder::symbol)
    pub fn build(self) -> Result<AccountBalanceToken, BuildError> {
        Ok(AccountBalanceToken {
            balance: self
                .balance
                .ok_or_else(|| BuildError::missing_field("balance"))?,
            breakdown: self
                .breakdown
                .ok_or_else(|| BuildError::missing_field("breakdown"))?,
            icon_url: self.icon_url,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            price_usd: self.price_usd,
            symbol: self
                .symbol
                .ok_or_else(|| BuildError::missing_field("symbol"))?,
            value_usd: self.value_usd,
        })
    }
}
