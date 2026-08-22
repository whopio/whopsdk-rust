pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransferOwnershipAccountsResponse {
    #[serde(default)]
    pub success: bool,
}

impl TransferOwnershipAccountsResponse {
    pub fn builder() -> TransferOwnershipAccountsResponseBuilder {
        <TransferOwnershipAccountsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferOwnershipAccountsResponseBuilder {
    success: Option<bool>,
}

impl TransferOwnershipAccountsResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransferOwnershipAccountsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](TransferOwnershipAccountsResponseBuilder::success)
    pub fn build(self) -> Result<TransferOwnershipAccountsResponse, BuildError> {
        Ok(TransferOwnershipAccountsResponse {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
