pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDepositsRequestDestinationAccountId {
    /// Destination account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Destination wallet address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Destination wallet network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<CreateDepositsRequestDestinationAccountIdNetwork>,
}

impl CreateDepositsRequestDestinationAccountId {
    pub fn builder() -> CreateDepositsRequestDestinationAccountIdBuilder {
        <CreateDepositsRequestDestinationAccountIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDepositsRequestDestinationAccountIdBuilder {
    account_id: Option<String>,
    address: Option<String>,
    network: Option<CreateDepositsRequestDestinationAccountIdNetwork>,
}

impl CreateDepositsRequestDestinationAccountIdBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    pub fn network(mut self, value: CreateDepositsRequestDestinationAccountIdNetwork) -> Self {
        self.network = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateDepositsRequestDestinationAccountId`].
    pub fn build(self) -> Result<CreateDepositsRequestDestinationAccountId, BuildError> {
        Ok(CreateDepositsRequestDestinationAccountId {
            account_id: self.account_id,
            address: self.address,
            network: self.network,
        })
    }
}
