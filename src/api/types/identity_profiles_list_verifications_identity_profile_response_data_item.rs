pub use crate::prelude::*;

/// An identity verification session used to confirm a person or entity's identity for payout account eligibility.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListVerificationsIdentityProfileResponseDataItem {
    /// When the verification record was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The numeric id of the verification record.
    #[serde(default)]
    pub id: String,
    /// The most recent error code returned during verification. Null if no error has occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<VerificationErrorCodes>,
    /// A human-readable explanation of the most recent verification error. Null if no error has occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_reason: Option<String>,
    /// A URL the user can visit to complete the verification process. Null if the session does not require user interaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_url: Option<String>,
    /// The current status of this verification session.
    pub status: VerificationStatuses,
}

impl ListVerificationsIdentityProfileResponseDataItem {
    pub fn builder() -> ListVerificationsIdentityProfileResponseDataItemBuilder {
        <ListVerificationsIdentityProfileResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListVerificationsIdentityProfileResponseDataItemBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    last_error_code: Option<VerificationErrorCodes>,
    last_error_reason: Option<String>,
    session_url: Option<String>,
    status: Option<VerificationStatuses>,
}

impl ListVerificationsIdentityProfileResponseDataItemBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

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

    pub fn session_url(mut self, value: impl Into<String>) -> Self {
        self.session_url = Some(value.into());
        self
    }

    pub fn status(mut self, value: VerificationStatuses) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListVerificationsIdentityProfileResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ListVerificationsIdentityProfileResponseDataItemBuilder::created_at)
    /// - [`id`](ListVerificationsIdentityProfileResponseDataItemBuilder::id)
    /// - [`status`](ListVerificationsIdentityProfileResponseDataItemBuilder::status)
    pub fn build(self) -> Result<ListVerificationsIdentityProfileResponseDataItem, BuildError> {
        Ok(ListVerificationsIdentityProfileResponseDataItem {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_error_code: self.last_error_code,
            last_error_reason: self.last_error_reason,
            session_url: self.session_url,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
