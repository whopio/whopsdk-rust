pub use crate::prelude::*;

/// Query parameters for retrieveStatus
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetupIntentsRetrieveStatusQueryRequest {
    /// The setup's own `client_secret`, for a caller without a secret key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}

impl SetupIntentsRetrieveStatusQueryRequest {
    pub fn builder() -> SetupIntentsRetrieveStatusQueryRequestBuilder {
        <SetupIntentsRetrieveStatusQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetupIntentsRetrieveStatusQueryRequestBuilder {
    client_secret: Option<String>,
}

impl SetupIntentsRetrieveStatusQueryRequestBuilder {
    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.client_secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetupIntentsRetrieveStatusQueryRequest`].
    pub fn build(self) -> Result<SetupIntentsRetrieveStatusQueryRequest, BuildError> {
        Ok(SetupIntentsRetrieveStatusQueryRequest {
            client_secret: self.client_secret,
        })
    }
}
