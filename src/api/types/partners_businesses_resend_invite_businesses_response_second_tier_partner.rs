pub use crate::prelude::*;

/// The second-tier partner who earns on this business (referred the first-tier partner). Null if there is no active second-tier partner.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResendInviteBusinessesResponseSecondTierPartner {
    /// User ID, prefixed `user_`.
    #[serde(default)]
    pub id: String,
    /// The user's display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The user's profile picture.
    #[serde(default)]
    pub profile_picture: ResendInviteBusinessesResponseSecondTierPartnerProfilePicture,
    /// The user's unique username.
    #[serde(default)]
    pub username: String,
}

impl ResendInviteBusinessesResponseSecondTierPartner {
    pub fn builder() -> ResendInviteBusinessesResponseSecondTierPartnerBuilder {
        <ResendInviteBusinessesResponseSecondTierPartnerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResendInviteBusinessesResponseSecondTierPartnerBuilder {
    id: Option<String>,
    name: Option<String>,
    profile_picture: Option<ResendInviteBusinessesResponseSecondTierPartnerProfilePicture>,
    username: Option<String>,
}

impl ResendInviteBusinessesResponseSecondTierPartnerBuilder {
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
        value: ResendInviteBusinessesResponseSecondTierPartnerProfilePicture,
    ) -> Self {
        self.profile_picture = Some(value);
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResendInviteBusinessesResponseSecondTierPartner`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ResendInviteBusinessesResponseSecondTierPartnerBuilder::id)
    /// - [`profile_picture`](ResendInviteBusinessesResponseSecondTierPartnerBuilder::profile_picture)
    /// - [`username`](ResendInviteBusinessesResponseSecondTierPartnerBuilder::username)
    pub fn build(self) -> Result<ResendInviteBusinessesResponseSecondTierPartner, BuildError> {
        Ok(ResendInviteBusinessesResponseSecondTierPartner {
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
