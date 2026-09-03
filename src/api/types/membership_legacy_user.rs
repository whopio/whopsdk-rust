pub use crate::prelude::*;

/// The user who owns this membership. Null if the user account has been deleted.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembershipLegacyUser {
    /// The user's email address. Requires the member:email:read permission to access. Null if not authorized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The unique identifier for the user.
    #[serde(default)]
    pub id: String,
    /// The user's display name shown on their public profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the user's profile picture. Use profilePicture for the full attachment object.
    #[serde(default)]
    pub profile_pic: String,
    /// The user's unique username shown on their public profile.
    #[serde(default)]
    pub username: String,
}

impl MembershipLegacyUser {
    pub fn builder() -> MembershipLegacyUserBuilder {
        <MembershipLegacyUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipLegacyUserBuilder {
    email: Option<String>,
    id: Option<String>,
    name: Option<String>,
    profile_pic: Option<String>,
    username: Option<String>,
}

impl MembershipLegacyUserBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn profile_pic(mut self, value: impl Into<String>) -> Self {
        self.profile_pic = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MembershipLegacyUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MembershipLegacyUserBuilder::id)
    /// - [`profile_pic`](MembershipLegacyUserBuilder::profile_pic)
    /// - [`username`](MembershipLegacyUserBuilder::username)
    pub fn build(self) -> Result<MembershipLegacyUser, BuildError> {
        Ok(MembershipLegacyUser {
            email: self.email,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            profile_pic: self
                .profile_pic
                .ok_or_else(|| BuildError::missing_field("profile_pic"))?,
            username: self
                .username
                .ok_or_else(|| BuildError::missing_field("username"))?,
        })
    }
}
