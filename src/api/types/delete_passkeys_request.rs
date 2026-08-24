pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeletePasskeysRequest {
    /// The `authenticatorData` from the WebAuthn assertion, base64url-encoded.
    #[serde(default)]
    pub authenticator_data: String,
    /// The `clientDataJSON` from the WebAuthn assertion, base64url-encoded.
    #[serde(default)]
    pub client_data_json: String,
    /// The `signature` from the WebAuthn assertion, base64url-encoded.
    #[serde(default)]
    pub signature: String,
}

impl DeletePasskeysRequest {
    pub fn builder() -> DeletePasskeysRequestBuilder {
        <DeletePasskeysRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeletePasskeysRequestBuilder {
    authenticator_data: Option<String>,
    client_data_json: Option<String>,
    signature: Option<String>,
}

impl DeletePasskeysRequestBuilder {
    pub fn authenticator_data(mut self, value: impl Into<String>) -> Self {
        self.authenticator_data = Some(value.into());
        self
    }

    pub fn client_data_json(mut self, value: impl Into<String>) -> Self {
        self.client_data_json = Some(value.into());
        self
    }

    pub fn signature(mut self, value: impl Into<String>) -> Self {
        self.signature = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeletePasskeysRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`authenticator_data`](DeletePasskeysRequestBuilder::authenticator_data)
    /// - [`client_data_json`](DeletePasskeysRequestBuilder::client_data_json)
    /// - [`signature`](DeletePasskeysRequestBuilder::signature)
    pub fn build(self) -> Result<DeletePasskeysRequest, BuildError> {
        Ok(DeletePasskeysRequest {
            authenticator_data: self
                .authenticator_data
                .ok_or_else(|| BuildError::missing_field("authenticator_data"))?,
            client_data_json: self
                .client_data_json
                .ok_or_else(|| BuildError::missing_field("client_data_json"))?,
            signature: self
                .signature
                .ok_or_else(|| BuildError::missing_field("signature"))?,
        })
    }
}
