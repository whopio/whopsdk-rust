pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateTransfersResponseClaimLinkSource {
    #[serde(default)]
    pub account_id: String,
}

impl CreateTransfersResponseClaimLinkSource {
    pub fn builder() -> CreateTransfersResponseClaimLinkSourceBuilder {
        <CreateTransfersResponseClaimLinkSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTransfersResponseClaimLinkSourceBuilder {
    account_id: Option<String>,
}

impl CreateTransfersResponseClaimLinkSourceBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateTransfersResponseClaimLinkSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateTransfersResponseClaimLinkSourceBuilder::account_id)
    pub fn build(self) -> Result<CreateTransfersResponseClaimLinkSource, BuildError> {
        Ok(CreateTransfersResponseClaimLinkSource {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
        })
    }
}
