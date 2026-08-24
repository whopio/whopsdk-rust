pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReferredUsersPartnersResponseDataItemUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_picture: Option<ReferredUsersPartnersResponseDataItemUserProfilePicture>,
    #[serde(default)]
    pub username: String,
}

impl ReferredUsersPartnersResponseDataItemUser {
    pub fn builder() -> ReferredUsersPartnersResponseDataItemUserBuilder {
        <ReferredUsersPartnersResponseDataItemUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReferredUsersPartnersResponseDataItemUserBuilder {
    city: Option<String>,
    country: Option<String>,
    id: Option<String>,
    name: Option<String>,
    profile_picture: Option<ReferredUsersPartnersResponseDataItemUserProfilePicture>,
    username: Option<String>,
}

impl ReferredUsersPartnersResponseDataItemUserBuilder {
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

    pub fn profile_picture(
        mut self,
        value: ReferredUsersPartnersResponseDataItemUserProfilePicture,
    ) -> Self {
        self.profile_picture = Some(value);
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReferredUsersPartnersResponseDataItemUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReferredUsersPartnersResponseDataItemUserBuilder::id)
    /// - [`username`](ReferredUsersPartnersResponseDataItemUserBuilder::username)
    pub fn build(self) -> Result<ReferredUsersPartnersResponseDataItemUser, BuildError> {
        Ok(ReferredUsersPartnersResponseDataItemUser {
            city: self.city,
            country: self.country,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            profile_picture: self.profile_picture,
            username: self
                .username
                .ok_or_else(|| BuildError::missing_field("username"))?,
        })
    }
}
