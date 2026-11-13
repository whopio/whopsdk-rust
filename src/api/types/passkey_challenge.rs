pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PasskeyChallenge {
    /// The challenge to pass to the WebAuthn ceremony, base64url-encoded without padding. Also inside the options.
    #[serde(default)]
    pub challenge: String,
    /// The options for `navigator.credentials.create()` when `challenge_type` is `registration`, otherwise `null`. Convert the keys to camelCase before passing them to the browser.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_options: Option<PasskeyCreationOptions>,
    /// The options for `navigator.credentials.get()` when `challenge_type` is `deletion`, otherwise `null`. Convert the keys to camelCase before passing them to the browser.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_options: Option<PasskeyRequestOptions>,
}

impl PasskeyChallenge {
    pub fn builder() -> PasskeyChallengeBuilder {
        <PasskeyChallengeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PasskeyChallengeBuilder {
    challenge: Option<String>,
    creation_options: Option<PasskeyCreationOptions>,
    request_options: Option<PasskeyRequestOptions>,
}

impl PasskeyChallengeBuilder {
    pub fn challenge(mut self, value: impl Into<String>) -> Self {
        self.challenge = Some(value.into());
        self
    }

    pub fn creation_options(mut self, value: PasskeyCreationOptions) -> Self {
        self.creation_options = Some(value);
        self
    }

    pub fn request_options(mut self, value: PasskeyRequestOptions) -> Self {
        self.request_options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PasskeyChallenge`].
    /// This method will fail if any of the following fields are not set:
    /// - [`challenge`](PasskeyChallengeBuilder::challenge)
    pub fn build(self) -> Result<PasskeyChallenge, BuildError> {
        Ok(PasskeyChallenge {
            challenge: self
                .challenge
                .ok_or_else(|| BuildError::missing_field("challenge"))?,
            creation_options: self.creation_options,
            request_options: self.request_options,
        })
    }
}
