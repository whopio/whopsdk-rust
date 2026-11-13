pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PartnerReferralRequest {
    /// Business receiving the request, when one is assigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountSummary>,
    /// The partner's team membership this referral was created for, prefixed `ausr_`. `null` when the referral is personal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_user_id: Option<String>,
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
    /// The partner's member record this referral was created for, prefixed `mber_`, when their role on that business is held on their member record. `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// Public profile of the requesting partner.
    #[serde(default)]
    pub partner: UserSummary,
    /// Business the partner sent this request on behalf of, when the referral belongs to their team. `null` for a personal referral.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referring_account: Option<AccountSummary>,
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
    /// User receiving the attribution request, or null for a business request or referral link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserSummary>,
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
    authorized_user_id: Option<String>,
    code: Option<String>,
    created_at: Option<String>,
    id: Option<String>,
    max_redemptions: Option<i64>,
    member_id: Option<String>,
    partner: Option<UserSummary>,
    referring_account: Option<AccountSummary>,
    request_type: Option<PartnerReferralRequestRequestType>,
    rewards: Option<Vec<PartnerReferralReward>>,
    status: Option<PartnerReferralRequestStatus>,
    updated_at: Option<String>,
    user: Option<UserSummary>,
}

impl PartnerReferralRequestBuilder {
    pub fn account(mut self, value: AccountSummary) -> Self {
        self.account = Some(value);
        self
    }

    pub fn authorized_user_id(mut self, value: impl Into<String>) -> Self {
        self.authorized_user_id = Some(value.into());
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

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn partner(mut self, value: UserSummary) -> Self {
        self.partner = Some(value);
        self
    }

    pub fn referring_account(mut self, value: AccountSummary) -> Self {
        self.referring_account = Some(value);
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

    pub fn user(mut self, value: UserSummary) -> Self {
        self.user = Some(value);
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
            authorized_user_id: self.authorized_user_id,
            code: self.code,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            max_redemptions: self.max_redemptions,
            member_id: self.member_id,
            partner: self
                .partner
                .ok_or_else(|| BuildError::missing_field("partner"))?,
            referring_account: self.referring_account,
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
            user: self.user,
        })
    }
}
