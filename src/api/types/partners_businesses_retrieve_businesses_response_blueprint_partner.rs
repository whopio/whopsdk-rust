pub use crate::prelude::*;

/// The partner whose blueprint the business deployed. Null unless this is a blueprint referral.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrieveBusinessesResponseBlueprintPartner {
    /// User ID, prefixed `user_`.
    #[serde(default)]
    pub id: String,
    /// The user's display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The user's profile picture.
    #[serde(default)]
    pub profile_picture: RetrieveBusinessesResponseBlueprintPartnerProfilePicture,
    /// The user's unique username.
    #[serde(default)]
    pub username: String,
}

impl RetrieveBusinessesResponseBlueprintPartner {
    pub fn builder() -> RetrieveBusinessesResponseBlueprintPartnerBuilder {
        <RetrieveBusinessesResponseBlueprintPartnerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveBusinessesResponseBlueprintPartnerBuilder {
    id: Option<String>,
    name: Option<String>,
    profile_picture: Option<RetrieveBusinessesResponseBlueprintPartnerProfilePicture>,
    username: Option<String>,
}

impl RetrieveBusinessesResponseBlueprintPartnerBuilder {
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
        value: RetrieveBusinessesResponseBlueprintPartnerProfilePicture,
    ) -> Self {
        self.profile_picture = Some(value);
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrieveBusinessesResponseBlueprintPartner`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RetrieveBusinessesResponseBlueprintPartnerBuilder::id)
    /// - [`profile_picture`](RetrieveBusinessesResponseBlueprintPartnerBuilder::profile_picture)
    /// - [`username`](RetrieveBusinessesResponseBlueprintPartnerBuilder::username)
    pub fn build(self) -> Result<RetrieveBusinessesResponseBlueprintPartner, BuildError> {
        Ok(RetrieveBusinessesResponseBlueprintPartner {
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
