pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PartnerReferralRequest {
    /// Business receiving the request, when one is assigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountSummary>,
    /// Unique referral code, when assigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// When the request was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Partner referral request ID, prefixed `prfr_`.
    #[serde(default)]
    pub id: String,
    /// Maximum permitted redemptions, when limited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redemptions: Option<i64>,
    /// Public profile of the requesting partner.
    #[serde(default)]
    pub partner: UserSummary,
    /// How the referral request was initiated.
    pub request_type: PartnerReferralRequestRequestType,
    #[serde(default)]
    pub rewards: Vec<PartnerReferralReward>,
    /// The approval state, or null for requests without an approval process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PartnerReferralRequestStatus>,
    /// When the request last changed, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl PartnerReferralRequest {
    pub fn builder() -> PartnerReferralRequestBuilder {
        <PartnerReferralRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnerReferralRequestBuilder {
    account: Option<AccountSummary>,
    code: Option<String>,
    created_at: Option<String>,
    id: Option<String>,
    max_redemptions: Option<i64>,
    partner: Option<UserSummary>,
    request_type: Option<PartnerReferralRequestRequestType>,
    rewards: Option<Vec<PartnerReferralReward>>,
    status: Option<PartnerReferralRequestStatus>,
    updated_at: Option<String>,
}

impl PartnerReferralRequestBuilder {
    pub fn account(mut self, value: AccountSummary) -> Self {
        self.account = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn max_redemptions(mut self, value: i64) -> Self {
        self.max_redemptions = Some(value);
        self
    }

    pub fn partner(mut self, value: UserSummary) -> Self {
        self.partner = Some(value);
        self
    }

    pub fn request_type(mut self, value: PartnerReferralRequestRequestType) -> Self {
        self.request_type = Some(value);
        self
    }

    pub fn rewards(mut self, value: Vec<PartnerReferralReward>) -> Self {
        self.rewards = Some(value);
        self
    }

    pub fn status(mut self, value: PartnerReferralRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PartnerReferralRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](PartnerReferralRequestBuilder::created_at)
    /// - [`id`](PartnerReferralRequestBuilder::id)
    /// - [`partner`](PartnerReferralRequestBuilder::partner)
    /// - [`request_type`](PartnerReferralRequestBuilder::request_type)
    /// - [`rewards`](PartnerReferralRequestBuilder::rewards)
    /// - [`updated_at`](PartnerReferralRequestBuilder::updated_at)
    pub fn build(self) -> Result<PartnerReferralRequest, BuildError> {
        Ok(PartnerReferralRequest {
            account: self.account,
            code: self.code,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            max_redemptions: self.max_redemptions,
            partner: self
                .partner
                .ok_or_else(|| BuildError::missing_field("partner"))?,
            request_type: self
                .request_type
                .ok_or_else(|| BuildError::missing_field("request_type"))?,
            rewards: self
                .rewards
                .ok_or_else(|| BuildError::missing_field("rewards"))?,
            status: self.status,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
