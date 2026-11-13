pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePartnerReferralRequestsRequestBodyTargetUsername {
    /// Your team membership on the business this referral is made for, prefixed `ausr_`. Omit for a personal referral.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_user_id: Option<String>,
    /// Your member record on the business this referral is made for, prefixed `mber_`, when your role there is held on your member record. Send this or `authorized_user_id`, not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// Username of the user to request attribution for, with or without a leading @.
    #[serde(default)]
    pub target_username: String,
}

impl CreatePartnerReferralRequestsRequestBodyTargetUsername {
    pub fn builder() -> CreatePartnerReferralRequestsRequestBodyTargetUsernameBuilder {
        <CreatePartnerReferralRequestsRequestBodyTargetUsernameBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePartnerReferralRequestsRequestBodyTargetUsernameBuilder {
    authorized_user_id: Option<String>,
    member_id: Option<String>,
    target_username: Option<String>,
}

impl CreatePartnerReferralRequestsRequestBodyTargetUsernameBuilder {
    pub fn authorized_user_id(mut self, value: impl Into<String>) -> Self {
        self.authorized_user_id = Some(value.into());
        self
    }

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn target_username(mut self, value: impl Into<String>) -> Self {
        self.target_username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePartnerReferralRequestsRequestBodyTargetUsername`].
    /// This method will fail if any of the following fields are not set:
    /// - [`target_username`](CreatePartnerReferralRequestsRequestBodyTargetUsernameBuilder::target_username)
    pub fn build(
        self,
    ) -> Result<CreatePartnerReferralRequestsRequestBodyTargetUsername, BuildError> {
        Ok(CreatePartnerReferralRequestsRequestBodyTargetUsername {
            authorized_user_id: self.authorized_user_id,
            member_id: self.member_id,
            target_username: self
                .target_username
                .ok_or_else(|| BuildError::missing_field("target_username"))?,
        })
    }
}
