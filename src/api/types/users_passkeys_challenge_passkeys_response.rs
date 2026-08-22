pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChallengePasskeysResponse {
    /// The challenge to pass to the WebAuthn ceremony, base64url-encoded without padding.
    #[serde(default)]
    pub challenge: String,
}

impl ChallengePasskeysResponse {
    pub fn builder() -> ChallengePasskeysResponseBuilder {
        <ChallengePasskeysResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChallengePasskeysResponseBuilder {
    challenge: Option<String>,
}

impl ChallengePasskeysResponseBuilder {
    pub fn challenge(mut self, value: impl Into<String>) -> Self {
        self.challenge = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChallengePasskeysResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`challenge`](ChallengePasskeysResponseBuilder::challenge)
    pub fn build(self) -> Result<ChallengePasskeysResponse, BuildError> {
        Ok(ChallengePasskeysResponse {
            challenge: self
                .challenge
                .ok_or_else(|| BuildError::missing_field("challenge"))?,
        })
    }
}
