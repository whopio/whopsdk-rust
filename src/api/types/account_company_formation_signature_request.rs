pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AccountCompanyFormationSignatureRequest {
    /// When the signing URL expires, as an ISO 8601 timestamp. Present while `status` is `pending`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// `pending` when a signing session is ready for the founder; `unknown` when the signature state could not be determined.
    pub status: AccountCompanyFormationSignatureRequestStatus,
    /// Hosted signing URL where the founder completes the form. Present while `status` is `pending`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl AccountCompanyFormationSignatureRequest {
    pub fn builder() -> AccountCompanyFormationSignatureRequestBuilder {
        <AccountCompanyFormationSignatureRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountCompanyFormationSignatureRequestBuilder {
    expires_at: Option<String>,
    status: Option<AccountCompanyFormationSignatureRequestStatus>,
    url: Option<String>,
}

impl AccountCompanyFormationSignatureRequestBuilder {
    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn status(mut self, value: AccountCompanyFormationSignatureRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountCompanyFormationSignatureRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](AccountCompanyFormationSignatureRequestBuilder::status)
    pub fn build(self) -> Result<AccountCompanyFormationSignatureRequest, BuildError> {
        Ok(AccountCompanyFormationSignatureRequest {
            expires_at: self.expires_at,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            url: self.url,
        })
    }
}
