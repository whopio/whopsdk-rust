pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainVerification {
    /// How far proving verification has got.
    pub state: DomainVerificationState,
    /// When an unproven claim is removed, 48 hours after it was made, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_expires_at: Option<String>,
}

impl DomainVerification {
    pub fn builder() -> DomainVerificationBuilder {
        <DomainVerificationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainVerificationBuilder {
    state: Option<DomainVerificationState>,
    verification_expires_at: Option<String>,
}

impl DomainVerificationBuilder {
    pub fn state(mut self, value: DomainVerificationState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn verification_expires_at(mut self, value: impl Into<String>) -> Self {
        self.verification_expires_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainVerification`].
    /// This method will fail if any of the following fields are not set:
    /// - [`state`](DomainVerificationBuilder::state)
    pub fn build(self) -> Result<DomainVerification, BuildError> {
        Ok(DomainVerification {
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
            verification_expires_at: self.verification_expires_at,
        })
    }
}
