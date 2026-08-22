pub use crate::prelude::*;

/// The partner who referred the business owner onto Whop (first tier). Null if there is no active first-tier partner.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrieveBusinessesResponseFirstTierPartner {
    /// User ID, prefixed `user_`.
    #[serde(default)]
    pub id: String,
    /// The user's display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The user's profile picture.
    #[serde(default)]
    pub profile_picture: RetrieveBusinessesResponseFirstTierPartnerProfilePicture,
    /// The user's unique username.
    #[serde(default)]
    pub username: String,
}

impl RetrieveBusinessesResponseFirstTierPartner {
    pub fn builder() -> RetrieveBusinessesResponseFirstTierPartnerBuilder {
        <RetrieveBusinessesResponseFirstTierPartnerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveBusinessesResponseFirstTierPartnerBuilder {
    id: Option<String>,
    name: Option<String>,
    profile_picture: Option<RetrieveBusinessesResponseFirstTierPartnerProfilePicture>,
    username: Option<String>,
}

impl RetrieveBusinessesResponseFirstTierPartnerBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn profile_picture(
        mut self,
        value: RetrieveBusinessesResponseFirstTierPartnerProfilePicture,
    ) -> Self {
        self.profile_picture = Some(value);
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrieveBusinessesResponseFirstTierPartner`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RetrieveBusinessesResponseFirstTierPartnerBuilder::id)
    /// - [`profile_picture`](RetrieveBusinessesResponseFirstTierPartnerBuilder::profile_picture)
    /// - [`username`](RetrieveBusinessesResponseFirstTierPartnerBuilder::username)
    pub fn build(self) -> Result<RetrieveBusinessesResponseFirstTierPartner, BuildError> {
        Ok(RetrieveBusinessesResponseFirstTierPartner {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            profile_picture: self
                .profile_picture
                .ok_or_else(|| BuildError::missing_field("profile_picture"))?,
            username: self
                .username
                .ok_or_else(|| BuildError::missing_field("username"))?,
        })
    }
}
