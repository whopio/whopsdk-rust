pub use crate::prelude::*;

/// Query parameters for retrieve
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConfirmationTokensRetrieveQueryRequest {
    /// The account (biz_) the token was minted for.
    #[serde(default)]
    pub account_id: String,
}

impl ConfirmationTokensRetrieveQueryRequest {
    pub fn builder() -> ConfirmationTokensRetrieveQueryRequestBuilder {
        <ConfirmationTokensRetrieveQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfirmationTokensRetrieveQueryRequestBuilder {
    account_id: Option<String>,
}

impl ConfirmationTokensRetrieveQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConfirmationTokensRetrieveQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](ConfirmationTokensRetrieveQueryRequestBuilder::account_id)
    pub fn build(self) -> Result<ConfirmationTokensRetrieveQueryRequest, BuildError> {
        Ok(ConfirmationTokensRetrieveQueryRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
        })
    }
}
