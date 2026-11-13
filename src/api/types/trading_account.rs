pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TradingAccount {
    /// The account that owns this trading account, prefixed `biz_`. `null` when a user owns it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Hyperliquid-specific state. Present when `provider` is `hyperliquid`, otherwise `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperliquid: Option<TradingHyperliquidAccount>,
    /// The Whop wallet ID backing this trading account, prefixed `cwal_`.
    #[serde(default)]
    pub id: String,
    pub object: TradingAccountObject,
    /// Trading venue that holds the positions and orders.
    pub provider: TradingAccountProvider,
    /// Spot holdings in the Whop wallet with their entry price and profit and loss. `null` when it could not be read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot: Option<TradingSpotAccount>,
    /// The user who owns this trading account, prefixed `user_`. `null` when an account owns it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl TradingAccount {
    pub fn builder() -> TradingAccountBuilder {
        <TradingAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TradingAccountBuilder {
    account_id: Option<String>,
    hyperliquid: Option<TradingHyperliquidAccount>,
    id: Option<String>,
    object: Option<TradingAccountObject>,
    provider: Option<TradingAccountProvider>,
    spot: Option<TradingSpotAccount>,
    user_id: Option<String>,
}

impl TradingAccountBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn hyperliquid(mut self, value: TradingHyperliquidAccount) -> Self {
        self.hyperliquid = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: TradingAccountObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn provider(mut self, value: TradingAccountProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn spot(mut self, value: TradingSpotAccount) -> Self {
        self.spot = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TradingAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TradingAccountBuilder::id)
    /// - [`object`](TradingAccountBuilder::object)
    /// - [`provider`](TradingAccountBuilder::provider)
    pub fn build(self) -> Result<TradingAccount, BuildError> {
        Ok(TradingAccount {
            account_id: self.account_id,
            hyperliquid: self.hyperliquid,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            spot: self.spot,
            user_id: self.user_id,
        })
    }
}
