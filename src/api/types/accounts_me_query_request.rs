pub use crate::prelude::*;

/// Query parameters for me
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountsMeQueryRequest {
    /// Also return the trading account under `trading`: its address, a snapshot of its margin and open positions, and its Hyperliquid WebSocket subscriptions. Requires crypto_wallet:trade:read, crypto_wallet:trade, or crypto_wallet:manage permission and an Ethereum wallet. Otherwise `trading` is `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_trading: Option<bool>,
}

impl AccountsMeQueryRequest {
    pub fn builder() -> AccountsMeQueryRequestBuilder {
        <AccountsMeQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsMeQueryRequestBuilder {
    include_trading: Option<bool>,
}

impl AccountsMeQueryRequestBuilder {
    pub fn include_trading(mut self, value: bool) -> Self {
        self.include_trading = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountsMeQueryRequest`].
    pub fn build(self) -> Result<AccountsMeQueryRequest, BuildError> {
        Ok(AccountsMeQueryRequest {
            include_trading: self.include_trading,
        })
    }
}
