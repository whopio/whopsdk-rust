pub use crate::prelude::*;

/// The second-tier partner who earns on this business (referred the first-tier partner). Null if there is no active second-tier partner.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrieveBusinessesResponseSecondTierPartner {
    /// User ID, prefixed `user_`.
    #[serde(default)]
    pub id: String,
    /// The user's display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The user's profile picture.
    #[serde(default)]
    pub profile_picture: RetrieveBusinessesResponseSecondTierPartnerProfilePicture,
    /// The user's unique username.
    #[serde(default)]
    pub username: String,
}

impl RetrieveBusinessesResponseSecondTierPartner {
    pub fn builder() -> RetrieveBusinessesResponseSecondTierPartnerBuilder {
        <RetrieveBusinessesResponseSecondTierPartnerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveBusinessesResponseSecondTierPartnerBuilder {
    id: Option<String>,
    name: Option<String>,
    profile_picture: Option<RetrieveBusinessesResponseSecondTierPartnerProfilePicture>,
    username: Option<String>,
}

impl RetrieveBusinessesResponseSecondTierPartnerBuilder {
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
        value: RetrieveBusinessesResponseSecondTierPartnerProfilePicture,
    ) -> Self {
        self.profile_picture = Some(value);
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrieveBusinessesResponseSecondTierPartner`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RetrieveBusinessesResponseSecondTierPartnerBuilder::id)
    /// - [`profile_picture`](RetrieveBusinessesResponseSecondTierPartnerBuilder::profile_picture)
    /// - [`username`](RetrieveBusinessesResponseSecondTierPartnerBuilder::username)
    pub fn build(self) -> Result<RetrieveBusinessesResponseSecondTierPartner, BuildError> {
        Ok(RetrieveBusinessesResponseSecondTierPartner {
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
