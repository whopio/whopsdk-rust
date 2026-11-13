pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateReturnUrlPaymentsRequest {
    /// The payment's own `client_secret`, for a caller without a secret key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// Where the buyer continues after completing an off-site step. Must be an absolute https URL without credentials (http is allowed for localhost), at most 2,048 characters.
    #[serde(default)]
    pub return_url: String,
}

impl UpdateReturnUrlPaymentsRequest {
    pub fn builder() -> UpdateReturnUrlPaymentsRequestBuilder {
        <UpdateReturnUrlPaymentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateReturnUrlPaymentsRequestBuilder {
    client_secret: Option<String>,
    return_url: Option<String>,
}

impl UpdateReturnUrlPaymentsRequestBuilder {
    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.client_secret = Some(value.into());
        self
    }

    pub fn return_url(mut self, value: impl Into<String>) -> Self {
        self.return_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateReturnUrlPaymentsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`return_url`](UpdateReturnUrlPaymentsRequestBuilder::return_url)
    pub fn build(self) -> Result<UpdateReturnUrlPaymentsRequest, BuildError> {
        Ok(UpdateReturnUrlPaymentsRequest {
            client_secret: self.client_secret,
            return_url: self
                .return_url
                .ok_or_else(|| BuildError::missing_field("return_url"))?,
        })
    }
}
