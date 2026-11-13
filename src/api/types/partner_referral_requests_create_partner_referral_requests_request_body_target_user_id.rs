pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePartnerReferralRequestsRequestBodyTargetUserId {
    /// Your team membership on the business this referral is made for, prefixed `ausr_`. Omit for a personal referral.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_user_id: Option<String>,
    /// Your member record on the business this referral is made for, prefixed `mber_`, when your role there is held on your member record. Send this or `authorized_user_id`, not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// User to request attribution for, prefixed `user_`.
    #[serde(default)]
    pub target_user_id: String,
}

impl CreatePartnerReferralRequestsRequestBodyTargetUserId {
    pub fn builder() -> CreatePartnerReferralRequestsRequestBodyTargetUserIdBuilder {
        <CreatePartnerReferralRequestsRequestBodyTargetUserIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePartnerReferralRequestsRequestBodyTargetUserIdBuilder {
    authorized_user_id: Option<String>,
    member_id: Option<String>,
    target_user_id: Option<String>,
}

impl CreatePartnerReferralRequestsRequestBodyTargetUserIdBuilder {
    pub fn authorized_user_id(mut self, value: impl Into<String>) -> Self {
        self.authorized_user_id = Some(value.into());
        self
    }

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn target_user_id(mut self, value: impl Into<String>) -> Self {
        self.target_user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePartnerReferralRequestsRequestBodyTargetUserId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`target_user_id`](CreatePartnerReferralRequestsRequestBodyTargetUserIdBuilder::target_user_id)
    pub fn build(self) -> Result<CreatePartnerReferralRequestsRequestBodyTargetUserId, BuildError> {
        Ok(CreatePartnerReferralRequestsRequestBodyTargetUserId {
            authorized_user_id: self.authorized_user_id,
            member_id: self.member_id,
            target_user_id: self
                .target_user_id
                .ok_or_else(|| BuildError::missing_field("target_user_id"))?,
        })
    }
}
