pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ChallengePasskeysRequest {
    /// The ceremony this challenge is for.
    pub challenge_type: ChallengePasskeysRequestChallengeType,
    /// The passkey the ceremony targets, prefixed `wcred_`. Required when `challenge_type` is `deletion`, ignored otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passkey_id: Option<String>,
}

impl ChallengePasskeysRequest {
    pub fn builder() -> ChallengePasskeysRequestBuilder {
        <ChallengePasskeysRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChallengePasskeysRequestBuilder {
    challenge_type: Option<ChallengePasskeysRequestChallengeType>,
    passkey_id: Option<String>,
}

impl ChallengePasskeysRequestBuilder {
    pub fn challenge_type(mut self, value: ChallengePasskeysRequestChallengeType) -> Self {
        self.challenge_type = Some(value);
        self
    }

    pub fn passkey_id(mut self, value: impl Into<String>) -> Self {
        self.passkey_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChallengePasskeysRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`challenge_type`](ChallengePasskeysRequestBuilder::challenge_type)
    pub fn build(self) -> Result<ChallengePasskeysRequest, BuildError> {
        Ok(ChallengePasskeysRequest {
            challenge_type: self
                .challenge_type
                .ok_or_else(|| BuildError::missing_field("challenge_type"))?,
            passkey_id: self.passkey_id,
        })
    }
}
