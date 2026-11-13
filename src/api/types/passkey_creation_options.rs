pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PasskeyCreationOptions {
    /// Whop does not ask for attestation.
    pub attestation: PasskeyCreationOptionsAttestation,
    /// What the authenticator must support.
    pub authenticator_selection: PasskeyAuthenticatorSelection,
    /// The registration challenge, base64url-encoded without padding.
    #[serde(default)]
    pub challenge: String,
    #[serde(default)]
    pub exclude_credentials: Vec<PasskeyCredentialDescriptor>,
    /// The WebAuthn extensions to run.
    #[serde(default)]
    pub extensions: PasskeyCreationExtensions,
    #[serde(default)]
    pub pub_key_cred_params: Vec<PasskeyCredentialParameter>,
    /// The site the passkey belongs to.
    #[serde(default)]
    pub rp: PasskeyRelyingParty,
    /// How long the ceremony may take, in milliseconds. Matches the challenge's lifetime.
    #[serde(default)]
    pub timeout: i64,
    /// The account the passkey signs in to.
    #[serde(default)]
    pub user: PasskeyUserEntity,
}

impl PasskeyCreationOptions {
    pub fn builder() -> PasskeyCreationOptionsBuilder {
        <PasskeyCreationOptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PasskeyCreationOptionsBuilder {
    attestation: Option<PasskeyCreationOptionsAttestation>,
    authenticator_selection: Option<PasskeyAuthenticatorSelection>,
    challenge: Option<String>,
    exclude_credentials: Option<Vec<PasskeyCredentialDescriptor>>,
    extensions: Option<PasskeyCreationExtensions>,
    pub_key_cred_params: Option<Vec<PasskeyCredentialParameter>>,
    rp: Option<PasskeyRelyingParty>,
    timeout: Option<i64>,
    user: Option<PasskeyUserEntity>,
}

impl PasskeyCreationOptionsBuilder {
    pub fn attestation(mut self, value: PasskeyCreationOptionsAttestation) -> Self {
        self.attestation = Some(value);
        self
    }

    pub fn authenticator_selection(mut self, value: PasskeyAuthenticatorSelection) -> Self {
        self.authenticator_selection = Some(value);
        self
    }

    pub fn challenge(mut self, value: impl Into<String>) -> Self {
        self.challenge = Some(value.into());
        self
    }

    pub fn exclude_credentials(mut self, value: Vec<PasskeyCredentialDescriptor>) -> Self {
        self.exclude_credentials = Some(value);
        self
    }

    pub fn extensions(mut self, value: PasskeyCreationExtensions) -> Self {
        self.extensions = Some(value);
        self
    }

    pub fn pub_key_cred_params(mut self, value: Vec<PasskeyCredentialParameter>) -> Self {
        self.pub_key_cred_params = Some(value);
        self
    }

    pub fn rp(mut self, value: PasskeyRelyingParty) -> Self {
        self.rp = Some(value);
        self
    }

    pub fn timeout(mut self, value: i64) -> Self {
        self.timeout = Some(value);
        self
    }

    pub fn user(mut self, value: PasskeyUserEntity) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PasskeyCreationOptions`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attestation`](PasskeyCreationOptionsBuilder::attestation)
    /// - [`authenticator_selection`](PasskeyCreationOptionsBuilder::authenticator_selection)
    /// - [`challenge`](PasskeyCreationOptionsBuilder::challenge)
    /// - [`exclude_credentials`](PasskeyCreationOptionsBuilder::exclude_credentials)
    /// - [`extensions`](PasskeyCreationOptionsBuilder::extensions)
    /// - [`pub_key_cred_params`](PasskeyCreationOptionsBuilder::pub_key_cred_params)
    /// - [`rp`](PasskeyCreationOptionsBuilder::rp)
    /// - [`timeout`](PasskeyCreationOptionsBuilder::timeout)
    /// - [`user`](PasskeyCreationOptionsBuilder::user)
    pub fn build(self) -> Result<PasskeyCreationOptions, BuildError> {
        Ok(PasskeyCreationOptions {
            attestation: self
                .attestation
                .ok_or_else(|| BuildError::missing_field("attestation"))?,
            authenticator_selection: self
                .authenticator_selection
                .ok_or_else(|| BuildError::missing_field("authenticator_selection"))?,
            challenge: self
                .challenge
                .ok_or_else(|| BuildError::missing_field("challenge"))?,
            exclude_credentials: self
                .exclude_credentials
                .ok_or_else(|| BuildError::missing_field("exclude_credentials"))?,
            extensions: self
                .extensions
                .ok_or_else(|| BuildError::missing_field("extensions"))?,
            pub_key_cred_params: self
                .pub_key_cred_params
                .ok_or_else(|| BuildError::missing_field("pub_key_cred_params"))?,
            rp: self.rp.ok_or_else(|| BuildError::missing_field("rp"))?,
            timeout: self
                .timeout
                .ok_or_else(|| BuildError::missing_field("timeout"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}
