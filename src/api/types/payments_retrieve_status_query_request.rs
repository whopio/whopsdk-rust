pub use crate::prelude::*;

/// Query parameters for retrieveStatus
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentsRetrieveStatusQueryRequest {
    /// The payment's own `client_secret`, for a caller without a secret key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}

impl PaymentsRetrieveStatusQueryRequest {
    pub fn builder() -> PaymentsRetrieveStatusQueryRequestBuilder {
        <PaymentsRetrieveStatusQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentsRetrieveStatusQueryRequestBuilder {
    client_secret: Option<String>,
}

impl PaymentsRetrieveStatusQueryRequestBuilder {
    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.client_secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentsRetrieveStatusQueryRequest`].
    pub fn build(self) -> Result<PaymentsRetrieveStatusQueryRequest, BuildError> {
        Ok(PaymentsRetrieveStatusQueryRequest {
            client_secret: self.client_secret,
        })
    }
}
