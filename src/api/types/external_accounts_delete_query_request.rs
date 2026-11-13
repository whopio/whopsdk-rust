pub use crate::prelude::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExternalAccountsDeleteQueryRequest {
    /// The Account that the external account is connected to. Provide either this or user_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The User that the external account is connected to. Provide either this or account_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl ExternalAccountsDeleteQueryRequest {
    pub fn builder() -> ExternalAccountsDeleteQueryRequestBuilder {
        <ExternalAccountsDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExternalAccountsDeleteQueryRequestBuilder {
    account_id: Option<String>,
    user_id: Option<String>,
}

impl ExternalAccountsDeleteQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExternalAccountsDeleteQueryRequest`].
    pub fn build(self) -> Result<ExternalAccountsDeleteQueryRequest, BuildError> {
        Ok(ExternalAccountsDeleteQueryRequest {
            account_id: self.account_id,
            user_id: self.user_id,
        })
    }
}
