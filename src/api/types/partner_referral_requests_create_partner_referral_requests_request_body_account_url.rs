pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePartnerReferralRequestsRequestBodyAccountUrl {
    /// HTTP or HTTPS Whop business or product link.
    #[serde(default)]
    pub account_url: String,
    /// Your team membership on the business this referral is made for, prefixed `ausr_`. Omit for a personal referral.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_user_id: Option<String>,
    /// Your member record on the business this referral is made for, prefixed `mber_`, when your role there is held on your member record. Send this or `authorized_user_id`, not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
}

impl CreatePartnerReferralRequestsRequestBodyAccountUrl {
    pub fn builder() -> CreatePartnerReferralRequestsRequestBodyAccountUrlBuilder {
        <CreatePartnerReferralRequestsRequestBodyAccountUrlBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePartnerReferralRequestsRequestBodyAccountUrlBuilder {
    account_url: Option<String>,
    authorized_user_id: Option<String>,
    member_id: Option<String>,
}

impl CreatePartnerReferralRequestsRequestBodyAccountUrlBuilder {
    pub fn account_url(mut self, value: impl Into<String>) -> Self {
        self.account_url = Some(value.into());
        self
    }

    pub fn authorized_user_id(mut self, value: impl Into<String>) -> Self {
        self.authorized_user_id = Some(value.into());
        self
    }

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePartnerReferralRequestsRequestBodyAccountUrl`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_url`](CreatePartnerReferralRequestsRequestBodyAccountUrlBuilder::account_url)
    pub fn build(self) -> Result<CreatePartnerReferralRequestsRequestBodyAccountUrl, BuildError> {
        Ok(CreatePartnerReferralRequestsRequestBodyAccountUrl {
            account_url: self
                .account_url
                .ok_or_else(|| BuildError::missing_field("account_url"))?,
            authorized_user_id: self.authorized_user_id,
            member_id: self.member_id,
        })
    }
}
