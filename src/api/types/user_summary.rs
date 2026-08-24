pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserSummary {
    /// User ID, prefixed `user_`.
    #[serde(default)]
    pub id: String,
    /// Display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Avatar wrapper; its `url` is always present, using a generated placeholder when the user set no picture.
    #[serde(default)]
    pub profile_picture: UserProfilePicture,
    /// Public username.
    #[serde(default)]
    pub username: String,
}

impl UserSummary {
    pub fn builder() -> UserSummaryBuilder {
        <UserSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserSummaryBuilder {
    id: Option<String>,
    name: Option<String>,
    profile_picture: Option<UserProfilePicture>,
    username: Option<String>,
}

impl UserSummaryBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn profile_picture(mut self, value: UserProfilePicture) -> Self {
        self.profile_picture = Some(value);
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UserSummaryBuilder::id)
    /// - [`profile_picture`](UserSummaryBuilder::profile_picture)
    /// - [`username`](UserSummaryBuilder::username)
    pub fn build(self) -> Result<UserSummary, BuildError> {
        Ok(UserSummary {
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
