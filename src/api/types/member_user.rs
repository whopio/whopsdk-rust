pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MemberUser {
    /// The user's city, derived from their IP address. `null` when unknown or the user turned off location sharing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The user's country as an ISO 3166-1 alpha-2 code, like `US`, derived from their IP address. `null` when unknown or the user turned off location sharing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// User ID, prefixed `user_`.
    #[serde(default)]
    pub id: String,
    /// Display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Avatar wrapper. Its `url` is always present, and is a generated placeholder when the user set no picture.
    #[serde(default)]
    pub profile_picture: UserProfilePicture,
    /// The user's state or region code, like `CA`, derived from their IP address. `null` when unknown or the user turned off location sharing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Public username.
    #[serde(default)]
    pub username: String,
}

impl MemberUser {
    pub fn builder() -> MemberUserBuilder {
        <MemberUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemberUserBuilder {
    city: Option<String>,
    country: Option<String>,
    id: Option<String>,
    name: Option<String>,
    profile_picture: Option<UserProfilePicture>,
    state: Option<String>,
    username: Option<String>,
}

impl MemberUserBuilder {
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
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

    pub fn profile_picture(mut self, value: UserProfilePicture) -> Self {
        self.profile_picture = Some(value);
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MemberUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MemberUserBuilder::id)
    /// - [`profile_picture`](MemberUserBuilder::profile_picture)
    /// - [`username`](MemberUserBuilder::username)
    pub fn build(self) -> Result<MemberUser, BuildError> {
        Ok(MemberUser {
            city: self.city,
            country: self.country,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            profile_picture: self
                .profile_picture
                .ok_or_else(|| BuildError::missing_field("profile_picture"))?,
            state: self.state,
            username: self
                .username
                .ok_or_else(|| BuildError::missing_field("username"))?,
        })
    }
}
