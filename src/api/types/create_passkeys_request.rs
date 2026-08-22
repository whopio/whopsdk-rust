pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePasskeysRequest {
    /// The `attestationObject` from the WebAuthn attestation response, base64url-encoded.
    #[serde(default)]
    pub attestation_object: String,
    /// The `clientDataJSON` from the WebAuthn attestation response, base64url-encoded.
    #[serde(default)]
    pub client_data_json: String,
    /// The WebAuthn credential ID the authenticator returned, base64url-encoded.
    #[serde(default)]
    pub credential_id: String,
    /// A name for this passkey, usually the device it lives on. 255 characters or fewer.
    #[serde(default)]
    pub nickname: String,
}

impl CreatePasskeysRequest {
    pub fn builder() -> CreatePasskeysRequestBuilder {
        <CreatePasskeysRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePasskeysRequestBuilder {
    attestation_object: Option<String>,
    client_data_json: Option<String>,
    credential_id: Option<String>,
    nickname: Option<String>,
}

impl CreatePasskeysRequestBuilder {
    pub fn attestation_object(mut self, value: impl Into<String>) -> Self {
        self.attestation_object = Some(value.into());
        self
    }

    pub fn client_data_json(mut self, value: impl Into<String>) -> Self {
        self.client_data_json = Some(value.into());
        self
    }

    pub fn credential_id(mut self, value: impl Into<String>) -> Self {
        self.credential_id = Some(value.into());
        self
    }

    pub fn nickname(mut self, value: impl Into<String>) -> Self {
        self.nickname = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePasskeysRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attestation_object`](CreatePasskeysRequestBuilder::attestation_object)
    /// - [`client_data_json`](CreatePasskeysRequestBuilder::client_data_json)
    /// - [`credential_id`](CreatePasskeysRequestBuilder::credential_id)
    /// - [`nickname`](CreatePasskeysRequestBuilder::nickname)
    pub fn build(self) -> Result<CreatePasskeysRequest, BuildError> {
        Ok(CreatePasskeysRequest {
            attestation_object: self
                .attestation_object
                .ok_or_else(|| BuildError::missing_field("attestation_object"))?,
            client_data_json: self
                .client_data_json
                .ok_or_else(|| BuildError::missing_field("client_data_json"))?,
            credential_id: self
                .credential_id
                .ok_or_else(|| BuildError::missing_field("credential_id"))?,
            nickname: self
                .nickname
                .ok_or_else(|| BuildError::missing_field("nickname"))?,
        })
    }
}
