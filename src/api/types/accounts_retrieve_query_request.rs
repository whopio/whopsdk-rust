pub use crate::prelude::*;

/// Query parameters for retrieve
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountsRetrieveQueryRequest {
    /// Also return the trading account under `trading`: its address, a snapshot of its margin and open positions, and its Hyperliquid WebSocket subscriptions. Requires crypto_wallet:trade:read, crypto_wallet:trade, or crypto_wallet:manage permission and an Ethereum wallet. Otherwise `trading` is `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_trading: Option<bool>,
}

impl AccountsRetrieveQueryRequest {
    pub fn builder() -> AccountsRetrieveQueryRequestBuilder {
        <AccountsRetrieveQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsRetrieveQueryRequestBuilder {
    include_trading: Option<bool>,
}

impl AccountsRetrieveQueryRequestBuilder {
    pub fn include_trading(mut self, value: bool) -> Self {
        self.include_trading = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountsRetrieveQueryRequest`].
    pub fn build(self) -> Result<AccountsRetrieveQueryRequest, BuildError> {
        Ok(AccountsRetrieveQueryRequest {
            include_trading: self.include_trading,
        })
    }
}
