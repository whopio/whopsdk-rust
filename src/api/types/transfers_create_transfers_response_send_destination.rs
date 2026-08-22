pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateTransfersResponseSendDestination {
    #[serde(default)]
    pub account_id: String,
    #[serde(default)]
    pub address: String,
}

impl CreateTransfersResponseSendDestination {
    pub fn builder() -> CreateTransfersResponseSendDestinationBuilder {
        <CreateTransfersResponseSendDestinationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTransfersResponseSendDestinationBuilder {
    account_id: Option<String>,
    address: Option<String>,
}

impl CreateTransfersResponseSendDestinationBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateTransfersResponseSendDestination`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateTransfersResponseSendDestinationBuilder::account_id)
    /// - [`address`](CreateTransfersResponseSendDestinationBuilder::address)
    pub fn build(self) -> Result<CreateTransfersResponseSendDestination, BuildError> {
        Ok(CreateTransfersResponseSendDestination {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            address: self
                .address
                .ok_or_else(|| BuildError::missing_field("address"))?,
        })
    }
}
