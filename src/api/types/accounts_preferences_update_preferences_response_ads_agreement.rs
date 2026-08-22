pub use crate::prelude::*;

/// The account's Whop Ads services and payment authorization agreement. While `pending_signature`, campaign launch is blocked; sign by answering `requested_information` via `PATCH /verifications/{id}`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdatePreferencesResponseAdsAgreement {
    /// When the agreement was signed, as an ISO 8601 timestamp. `null` until signed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<String>,
    /// The agreement version signed or awaiting signature, as an ISO date. `null` when no signature is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_version: Option<String>,
    /// The signer's printed full name. `null` until signed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printed_name: Option<String>,
    /// Where the account's ads services agreement stands.
    pub status: UpdatePreferencesResponseAdsAgreementStatus,
}

impl UpdatePreferencesResponseAdsAgreement {
    pub fn builder() -> UpdatePreferencesResponseAdsAgreementBuilder {
        <UpdatePreferencesResponseAdsAgreementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePreferencesResponseAdsAgreementBuilder {
    accepted_at: Option<String>,
    agreement_version: Option<String>,
    printed_name: Option<String>,
    status: Option<UpdatePreferencesResponseAdsAgreementStatus>,
}

impl UpdatePreferencesResponseAdsAgreementBuilder {
    pub fn accepted_at(mut self, value: impl Into<String>) -> Self {
        self.accepted_at = Some(value.into());
        self
    }

    pub fn agreement_version(mut self, value: impl Into<String>) -> Self {
        self.agreement_version = Some(value.into());
        self
    }

    pub fn printed_name(mut self, value: impl Into<String>) -> Self {
        self.printed_name = Some(value.into());
        self
    }

    pub fn status(mut self, value: UpdatePreferencesResponseAdsAgreementStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePreferencesResponseAdsAgreement`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](UpdatePreferencesResponseAdsAgreementBuilder::status)
    pub fn build(self) -> Result<UpdatePreferencesResponseAdsAgreement, BuildError> {
        Ok(UpdatePreferencesResponseAdsAgreement {
            accepted_at: self.accepted_at,
            agreement_version: self.agreement_version,
            printed_name: self.printed_name,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
