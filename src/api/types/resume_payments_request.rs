pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResumePaymentsRequest {
    /// The payment's own `client_secret`, for a buyer without a token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// Where the buyer continues after completing an off-site step. Must be an absolute https URL without credentials (http is allowed for localhost), at most 2,048 characters. Defaults to Whop's hosted verification page. Ignored when the payment is not waiting on authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

impl ResumePaymentsRequest {
    pub fn builder() -> ResumePaymentsRequestBuilder {
        <ResumePaymentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResumePaymentsRequestBuilder {
    client_secret: Option<String>,
    return_url: Option<String>,
}

impl ResumePaymentsRequestBuilder {
    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.client_secret = Some(value.into());
        self
    }

    pub fn return_url(mut self, value: impl Into<String>) -> Self {
        self.return_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResumePaymentsRequest`].
    pub fn build(self) -> Result<ResumePaymentsRequest, BuildError> {
        Ok(ResumePaymentsRequest {
            client_secret: self.client_secret,
            return_url: self.return_url,
        })
    }
}
