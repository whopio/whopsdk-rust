pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReferredUsersPartnersResponseDataItemUserProfilePicture {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ReferredUsersPartnersResponseDataItemUserProfilePicture {
    pub fn builder() -> ReferredUsersPartnersResponseDataItemUserProfilePictureBuilder {
        <ReferredUsersPartnersResponseDataItemUserProfilePictureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReferredUsersPartnersResponseDataItemUserProfilePictureBuilder {
    url: Option<String>,
}

impl ReferredUsersPartnersResponseDataItemUserProfilePictureBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReferredUsersPartnersResponseDataItemUserProfilePicture`].
    pub fn build(
        self,
    ) -> Result<ReferredUsersPartnersResponseDataItemUserProfilePicture, BuildError> {
        Ok(ReferredUsersPartnersResponseDataItemUserProfilePicture { url: self.url })
    }
}
