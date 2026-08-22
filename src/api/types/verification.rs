pub use crate::prelude::*;

/// An identity verification session used to confirm a person or entity's identity for payout account eligibility.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Verification {
    /// The numeric id of the verification record.
    #[serde(default)]
    pub id: String,
    /// The most recent error code returned during verification. Null if no error has occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<VerificationErrorCodes>,
    /// A human-readable explanation of the most recent verification error. Null if no error has occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_reason: Option<String>,
    /// The current status of this verification session.
    pub status: VerificationStatuses,
}

impl Verification {
    pub fn builder() -> VerificationBuilder {
        <VerificationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VerificationBuilder {
    id: Option<String>,
    last_error_code: Option<VerificationErrorCodes>,
    last_error_reason: Option<String>,
    status: Option<VerificationStatuses>,
}

impl VerificationBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_error_code(mut self, value: VerificationErrorCodes) -> Self {
        self.last_error_code = Some(value);
        self
    }

    pub fn last_error_reason(mut self, value: impl Into<String>) -> Self {
        self.last_error_reason = Some(value.into());
        self
    }

    pub fn status(mut self, value: VerificationStatuses) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Verification`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](VerificationBuilder::id)
    /// - [`status`](VerificationBuilder::status)
    pub fn build(self) -> Result<Verification, BuildError> {
        Ok(Verification {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_error_code: self.last_error_code,
            last_error_reason: self.last_error_reason,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
