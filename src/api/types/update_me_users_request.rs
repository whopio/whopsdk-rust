pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMeUsersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<UpdateMeUsersRequestBanner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// The user's date of birth as an ISO 8601 date, like `1998-04-23`. The user must be at least 13. Global profile only; refused with `account_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_picture: Option<UpdateMeUsersRequestProfilePicture>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// When set, updates the authenticated user's profile override for this account instead of their global profile.
    #[serde(skip)]
    pub account_id: Option<String>,
}

impl UpdateMeUsersRequest {
    pub fn builder() -> UpdateMeUsersRequestBuilder {
        <UpdateMeUsersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMeUsersRequestBuilder {
    banner: Option<UpdateMeUsersRequestBanner>,
    bio: Option<String>,
    date_of_birth: Option<NaiveDate>,
    name: Option<String>,
    profile_picture: Option<UpdateMeUsersRequestProfilePicture>,
    username: Option<String>,
    account_id: Option<String>,
}

impl UpdateMeUsersRequestBuilder {
    pub fn banner(mut self, value: UpdateMeUsersRequestBanner) -> Self {
        self.banner = Some(value);
        self
    }

    pub fn bio(mut self, value: impl Into<String>) -> Self {
        self.bio = Some(value.into());
        self
    }

    pub fn date_of_birth(mut self, value: NaiveDate) -> Self {
        self.date_of_birth = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn profile_picture(mut self, value: UpdateMeUsersRequestProfilePicture) -> Self {
        self.profile_picture = Some(value);
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateMeUsersRequest`].
    pub fn build(self) -> Result<UpdateMeUsersRequest, BuildError> {
        Ok(UpdateMeUsersRequest {
            banner: self.banner,
            bio: self.bio,
            date_of_birth: self.date_of_birth,
            name: self.name,
            profile_picture: self.profile_picture,
            username: self.username,
            account_id: self.account_id,
        })
    }
}
