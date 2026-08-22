pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateTransfersResponseSendSource {
    #[serde(default)]
    pub account_id: String,
    #[serde(default)]
    pub address: String,
}

impl CreateTransfersResponseSendSource {
    pub fn builder() -> CreateTransfersResponseSendSourceBuilder {
        <CreateTransfersResponseSendSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTransfersResponseSendSourceBuilder {
    account_id: Option<String>,
    address: Option<String>,
}

impl CreateTransfersResponseSendSourceBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateTransfersResponseSendSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateTransfersResponseSendSourceBuilder::account_id)
    /// - [`address`](CreateTransfersResponseSendSourceBuilder::address)
    pub fn build(self) -> Result<CreateTransfersResponseSendSource, BuildError> {
        Ok(CreateTransfersResponseSendSource {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            address: self
                .address
                .ok_or_else(|| BuildError::missing_field("address"))?,
        })
    }
}
