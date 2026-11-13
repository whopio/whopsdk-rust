pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePartnerReferralRequestsRequestBodyTargetEmail {
    /// Your team membership on the business this referral is made for, prefixed `ausr_`. Omit for a personal referral.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_user_id: Option<String>,
    /// Your member record on the business this referral is made for, prefixed `mber_`, when your role there is held on your member record. Send this or `authorized_user_id`, not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// Email address on an existing user's account. Matching is case-insensitive. The user does not need to be enrolled in the partner program.
    #[serde(default)]
    pub target_email: String,
}

impl CreatePartnerReferralRequestsRequestBodyTargetEmail {
    pub fn builder() -> CreatePartnerReferralRequestsRequestBodyTargetEmailBuilder {
        <CreatePartnerReferralRequestsRequestBodyTargetEmailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePartnerReferralRequestsRequestBodyTargetEmailBuilder {
    authorized_user_id: Option<String>,
    member_id: Option<String>,
    target_email: Option<String>,
}

impl CreatePartnerReferralRequestsRequestBodyTargetEmailBuilder {
    pub fn authorized_user_id(mut self, value: impl Into<String>) -> Self {
        self.authorized_user_id = Some(value.into());
        self
    }

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn target_email(mut self, value: impl Into<String>) -> Self {
        self.target_email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePartnerReferralRequestsRequestBodyTargetEmail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`target_email`](CreatePartnerReferralRequestsRequestBodyTargetEmailBuilder::target_email)
    pub fn build(self) -> Result<CreatePartnerReferralRequestsRequestBodyTargetEmail, BuildError> {
        Ok(CreatePartnerReferralRequestsRequestBodyTargetEmail {
            authorized_user_id: self.authorized_user_id,
            member_id: self.member_id,
            target_email: self
                .target_email
                .ok_or_else(|| BuildError::missing_field("target_email"))?,
        })
    }
}
