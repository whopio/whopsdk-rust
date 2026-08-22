pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AccountWallet {
    /// The on-chain address of the wallet
    #[serde(default)]
    pub address: String,
    /// Wallet ID, prefixed `wallet_`.
    #[serde(default)]
    pub id: String,
    /// The blockchain network the wallet lives on
    pub network: AccountWalletNetwork,
}

impl AccountWallet {
    pub fn builder() -> AccountWalletBuilder {
        <AccountWalletBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountWalletBuilder {
    address: Option<String>,
    id: Option<String>,
    network: Option<AccountWalletNetwork>,
}

impl AccountWalletBuilder {
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn network(mut self, value: AccountWalletNetwork) -> Self {
        self.network = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountWallet`].
    /// This method will fail if any of the following fields are not set:
    /// - [`address`](AccountWalletBuilder::address)
    /// - [`id`](AccountWalletBuilder::id)
    /// - [`network`](AccountWalletBuilder::network)
    pub fn build(self) -> Result<AccountWallet, BuildError> {
        Ok(AccountWallet {
            address: self
                .address
                .ok_or_else(|| BuildError::missing_field("address"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            network: self
                .network
                .ok_or_else(|| BuildError::missing_field("network"))?,
        })
    }
}
