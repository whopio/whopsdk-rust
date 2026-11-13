pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePartnerReferralRequestsRequestBodyAccountId {
    /// Business to request attribution for, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// Your team membership on the business this referral is made for, prefixed `ausr_`. Omit for a personal referral.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_user_id: Option<String>,
    /// Your member record on the business this referral is made for, prefixed `mber_`, when your role there is held on your member record. Send this or `authorized_user_id`, not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
}

impl CreatePartnerReferralRequestsRequestBodyAccountId {
    pub fn builder() -> CreatePartnerReferralRequestsRequestBodyAccountIdBuilder {
        <CreatePartnerReferralRequestsRequestBodyAccountIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePartnerReferralRequestsRequestBodyAccountIdBuilder {
    account_id: Option<String>,
    authorized_user_id: Option<String>,
    member_id: Option<String>,
}

impl CreatePartnerReferralRequestsRequestBodyAccountIdBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`CreatePartnerReferralRequestsRequestBodyAccountId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreatePartnerReferralRequestsRequestBodyAccountIdBuilder::account_id)
    pub fn build(self) -> Result<CreatePartnerReferralRequestsRequestBodyAccountId, BuildError> {
        Ok(CreatePartnerReferralRequestsRequestBodyAccountId {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            authorized_user_id: self.authorized_user_id,
            member_id: self.member_id,
        })
    }
}
