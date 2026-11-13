pub use crate::prelude::*;

/// Create your own referral link with an optional custom code and redemption limit. Without configuration, returns your oldest saved link for the selected team membership, or your personal context when `authorized_user_id` is omitted, creating a random code when none exists.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreatePartnerReferralRequestsRequestBodyCode {
    /// The partner's team membership on the business this link is made for, prefixed `ausr_`. Omit for a personal link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_user_id: Option<String>,
    /// Case-insensitive referral code containing letters, numbers, and single hyphens. New codes are stored in lowercase. Omit to generate six random letters, or reuse a saved link when no configuration is supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Maximum permitted redemptions, or null for no configured limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redemptions: Option<i64>,
    /// The partner's member record on the business this link is made for, prefixed `mber_`, when their role there is held on their member record. Send this or `authorized_user_id`, not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// Optional. Your own partner ID, prefixed `user_`; you cannot create a link for another partner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    /// Create or retrieve a reusable referral link without requesting attribution for an existing business.
    pub request_type: CreatePartnerReferralRequestsRequestBodyCodeRequestType,
}

impl CreatePartnerReferralRequestsRequestBodyCode {
    pub fn builder() -> CreatePartnerReferralRequestsRequestBodyCodeBuilder {
        <CreatePartnerReferralRequestsRequestBodyCodeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePartnerReferralRequestsRequestBodyCodeBuilder {
    authorized_user_id: Option<String>,
    code: Option<String>,
    max_redemptions: Option<i64>,
    member_id: Option<String>,
    partner_id: Option<String>,
    request_type: Option<CreatePartnerReferralRequestsRequestBodyCodeRequestType>,
}

impl CreatePartnerReferralRequestsRequestBodyCodeBuilder {
    pub fn authorized_user_id(mut self, value: impl Into<String>) -> Self {
        self.authorized_user_id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
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

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn request_type(
        mut self,
        value: CreatePartnerReferralRequestsRequestBodyCodeRequestType,
    ) -> Self {
        self.request_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePartnerReferralRequestsRequestBodyCode`].
    /// This method will fail if any of the following fields are not set:
    /// - [`request_type`](CreatePartnerReferralRequestsRequestBodyCodeBuilder::request_type)
    pub fn build(self) -> Result<CreatePartnerReferralRequestsRequestBodyCode, BuildError> {
        Ok(CreatePartnerReferralRequestsRequestBodyCode {
            authorized_user_id: self.authorized_user_id,
            code: self.code,
            max_redemptions: self.max_redemptions,
            member_id: self.member_id,
            partner_id: self.partner_id,
            request_type: self
                .request_type
                .ok_or_else(|| BuildError::missing_field("request_type"))?,
        })
    }
}
