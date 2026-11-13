pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProductAffiliateUser {
    /// The affiliate's email address. Requires the `member:email:read` scope; `null` without it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// User ID, prefixed `user_`.
    #[serde(default)]
    pub id: String,
    /// Display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Avatar wrapper. Its `url` is always present, and is a generated placeholder when the user set no picture.
    #[serde(default)]
    pub profile_picture: UserProfilePicture,
    /// Public username.
    #[serde(default)]
    pub username: String,
}

impl ProductAffiliateUser {
    pub fn builder() -> ProductAffiliateUserBuilder {
        <ProductAffiliateUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductAffiliateUserBuilder {
    email: Option<String>,
    id: Option<String>,
    name: Option<String>,
    profile_picture: Option<UserProfilePicture>,
    username: Option<String>,
}

impl ProductAffiliateUserBuilder {
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

    pub fn profile_picture(mut self, value: UserProfilePicture) -> Self {
        self.profile_picture = Some(value);
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProductAffiliateUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ProductAffiliateUserBuilder::id)
    /// - [`profile_picture`](ProductAffiliateUserBuilder::profile_picture)
    /// - [`username`](ProductAffiliateUserBuilder::username)
    pub fn build(self) -> Result<ProductAffiliateUser, BuildError> {
        Ok(ProductAffiliateUser {
            email: self.email,
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
