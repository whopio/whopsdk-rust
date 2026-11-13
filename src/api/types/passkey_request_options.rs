pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PasskeyRequestOptions {
    #[serde(default)]
    pub allow_credentials: Vec<PasskeyCredentialDescriptor>,
    /// The assertion challenge, base64url-encoded without padding.
    #[serde(default)]
    pub challenge: String,
    /// The site the passkey belongs to.
    #[serde(default)]
    pub rp_id: String,
    /// How long the ceremony may take, in milliseconds. Matches the challenge's lifetime.
    #[serde(default)]
    pub timeout: i64,
    /// The authenticator must verify the user, with a biometric or the device's screen lock.
    pub user_verification: PasskeyRequestOptionsUserVerification,
}

impl PasskeyRequestOptions {
    pub fn builder() -> PasskeyRequestOptionsBuilder {
        <PasskeyRequestOptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PasskeyRequestOptionsBuilder {
    allow_credentials: Option<Vec<PasskeyCredentialDescriptor>>,
    challenge: Option<String>,
    rp_id: Option<String>,
    timeout: Option<i64>,
    user_verification: Option<PasskeyRequestOptionsUserVerification>,
}

impl PasskeyRequestOptionsBuilder {
    pub fn allow_credentials(mut self, value: Vec<PasskeyCredentialDescriptor>) -> Self {
        self.allow_credentials = Some(value);
        self
    }

    pub fn challenge(mut self, value: impl Into<String>) -> Self {
        self.challenge = Some(value.into());
        self
    }

    pub fn rp_id(mut self, value: impl Into<String>) -> Self {
        self.rp_id = Some(value.into());
        self
    }

    pub fn timeout(mut self, value: i64) -> Self {
        self.timeout = Some(value);
        self
    }

    pub fn user_verification(mut self, value: PasskeyRequestOptionsUserVerification) -> Self {
        self.user_verification = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PasskeyRequestOptions`].
    /// This method will fail if any of the following fields are not set:
    /// - [`allow_credentials`](PasskeyRequestOptionsBuilder::allow_credentials)
    /// - [`challenge`](PasskeyRequestOptionsBuilder::challenge)
    /// - [`rp_id`](PasskeyRequestOptionsBuilder::rp_id)
    /// - [`timeout`](PasskeyRequestOptionsBuilder::timeout)
    /// - [`user_verification`](PasskeyRequestOptionsBuilder::user_verification)
    pub fn build(self) -> Result<PasskeyRequestOptions, BuildError> {
        Ok(PasskeyRequestOptions {
            allow_credentials: self
                .allow_credentials
                .ok_or_else(|| BuildError::missing_field("allow_credentials"))?,
            challenge: self
                .challenge
                .ok_or_else(|| BuildError::missing_field("challenge"))?,
            rp_id: self
                .rp_id
                .ok_or_else(|| BuildError::missing_field("rp_id"))?,
            timeout: self
                .timeout
                .ok_or_else(|| BuildError::missing_field("timeout"))?,
            user_verification: self
                .user_verification
                .ok_or_else(|| BuildError::missing_field("user_verification"))?,
        })
    }
}
