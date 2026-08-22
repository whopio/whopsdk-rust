pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UserBalanceToken {
    /// Amount held in native token units, as a decimal string.
    #[serde(default)]
    pub balance: String,
    /// Balance split into available, pending, in-transit, and reserve amounts, as native-unit decimal strings. Transfers between the user's own wallet and card are reported in `in_transit` until they arrive.
    #[serde(default)]
    pub breakdown: AccountBalanceBreakdown,
    /// Token icon URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// The token's display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// USD price per token, or `null` when unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price_usd: Option<f64>,
    /// Token display symbol, such as `USDT`, `XAUT`, or `cbBTC`.
    #[serde(default)]
    pub symbol: String,
    /// Holding USD value.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub value_usd: f64,
}

impl UserBalanceToken {
    pub fn builder() -> UserBalanceTokenBuilder {
        <UserBalanceTokenBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserBalanceTokenBuilder {
    balance: Option<String>,
    breakdown: Option<AccountBalanceBreakdown>,
    icon_url: Option<String>,
    name: Option<String>,
    price_usd: Option<f64>,
    symbol: Option<String>,
    value_usd: Option<f64>,
}

impl UserBalanceTokenBuilder {
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

    pub fn value_usd(mut self, value: f64) -> Self {
        self.value_usd = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserBalanceToken`].
    /// This method will fail if any of the following fields are not set:
    /// - [`balance`](UserBalanceTokenBuilder::balance)
    /// - [`breakdown`](UserBalanceTokenBuilder::breakdown)
    /// - [`symbol`](UserBalanceTokenBuilder::symbol)
    /// - [`value_usd`](UserBalanceTokenBuilder::value_usd)
    pub fn build(self) -> Result<UserBalanceToken, BuildError> {
        Ok(UserBalanceToken {
            balance: self
                .balance
                .ok_or_else(|| BuildError::missing_field("balance"))?,
            breakdown: self
                .breakdown
                .ok_or_else(|| BuildError::missing_field("breakdown"))?,
            icon_url: self.icon_url,
            name: self.name,
            price_usd: self.price_usd,
            symbol: self
                .symbol
                .ok_or_else(|| BuildError::missing_field("symbol"))?,
            value_usd: self
                .value_usd
                .ok_or_else(|| BuildError::missing_field("value_usd"))?,
        })
    }
}
