pub use crate::prelude::*;

/// The owner of the referred business.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBusinessesResponseDataItemOwner {
    /// The business owner's email address, so a partner can follow up on a referral they made. Requires the `partner:email:read` scope; `null` without it, or while the account has no reachable address of its own.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// User ID, prefixed `user_`.
    #[serde(default)]
    pub id: String,
    /// The user's display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The user's profile picture.
    #[serde(default)]
    pub profile_picture: ListBusinessesResponseDataItemOwnerProfilePicture,
    /// The user's unique username.
    #[serde(default)]
    pub username: String,
}

impl ListBusinessesResponseDataItemOwner {
    pub fn builder() -> ListBusinessesResponseDataItemOwnerBuilder {
        <ListBusinessesResponseDataItemOwnerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBusinessesResponseDataItemOwnerBuilder {
    email: Option<String>,
    id: Option<String>,
    name: Option<String>,
    profile_picture: Option<ListBusinessesResponseDataItemOwnerProfilePicture>,
    username: Option<String>,
}

impl ListBusinessesResponseDataItemOwnerBuilder {
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

    pub fn profile_picture(
        mut self,
        value: ListBusinessesResponseDataItemOwnerProfilePicture,
    ) -> Self {
        self.profile_picture = Some(value);
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListBusinessesResponseDataItemOwner`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ListBusinessesResponseDataItemOwnerBuilder::id)
    /// - [`profile_picture`](ListBusinessesResponseDataItemOwnerBuilder::profile_picture)
    /// - [`username`](ListBusinessesResponseDataItemOwnerBuilder::username)
    pub fn build(self) -> Result<ListBusinessesResponseDataItemOwner, BuildError> {
        Ok(ListBusinessesResponseDataItemOwner {
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
