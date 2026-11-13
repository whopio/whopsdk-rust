pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TradingHyperliquidAccount {
    /// Lowercase wallet address that holds the Hyperliquid account.
    #[serde(default)]
    pub address: String,
    /// USDT0 in the Whop wallet on Plasma that a `deposit` trade can move into this account, as a decimal string. `null` when the chain could not be read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depositable: Option<String>,
    /// Account value and margin across the main exchange and the `xyz` markets, read from Hyperliquid at most once a second. `null` when Hyperliquid could not be read. Use `websocket_subscriptions` for live updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_summary: Option<TradingMarginSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<TradingPosition>>,
    #[serde(default)]
    pub websocket_subscriptions: Vec<TradingWebsocketSubscription>,
    /// Hyperliquid WebSocket URL to connect to directly for live updates.
    #[serde(default)]
    pub websocket_url: String,
}

impl TradingHyperliquidAccount {
    pub fn builder() -> TradingHyperliquidAccountBuilder {
        <TradingHyperliquidAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TradingHyperliquidAccountBuilder {
    address: Option<String>,
    depositable: Option<String>,
    margin_summary: Option<TradingMarginSummary>,
    positions: Option<Vec<TradingPosition>>,
    websocket_subscriptions: Option<Vec<TradingWebsocketSubscription>>,
    websocket_url: Option<String>,
}

impl TradingHyperliquidAccountBuilder {
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    pub fn depositable(mut self, value: impl Into<String>) -> Self {
        self.depositable = Some(value.into());
        self
    }

    pub fn margin_summary(mut self, value: TradingMarginSummary) -> Self {
        self.margin_summary = Some(value);
        self
    }

    pub fn positions(mut self, value: Vec<TradingPosition>) -> Self {
        self.positions = Some(value);
        self
    }

    pub fn websocket_subscriptions(mut self, value: Vec<TradingWebsocketSubscription>) -> Self {
        self.websocket_subscriptions = Some(value);
        self
    }

    pub fn websocket_url(mut self, value: impl Into<String>) -> Self {
        self.websocket_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TradingHyperliquidAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`address`](TradingHyperliquidAccountBuilder::address)
    /// - [`websocket_subscriptions`](TradingHyperliquidAccountBuilder::websocket_subscriptions)
    /// - [`websocket_url`](TradingHyperliquidAccountBuilder::websocket_url)
    pub fn build(self) -> Result<TradingHyperliquidAccount, BuildError> {
        Ok(TradingHyperliquidAccount {
            address: self
                .address
                .ok_or_else(|| BuildError::missing_field("address"))?,
            depositable: self.depositable,
            margin_summary: self.margin_summary,
            positions: self.positions,
            websocket_subscriptions: self
                .websocket_subscriptions
                .ok_or_else(|| BuildError::missing_field("websocket_subscriptions"))?,
            websocket_url: self
                .websocket_url
                .ok_or_else(|| BuildError::missing_field("websocket_url"))?,
        })
    }
}
