pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserProfilePicture {
    /// Avatar image URL. Always present — a generated placeholder when the user set no picture.
    #[serde(default)]
    pub url: String,
}

impl UserProfilePicture {
    pub fn builder() -> UserProfilePictureBuilder {
        <UserProfilePictureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserProfilePictureBuilder {
    url: Option<String>,
}

impl UserProfilePictureBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserProfilePicture`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](UserProfilePictureBuilder::url)
    pub fn build(self) -> Result<UserProfilePicture, BuildError> {
        Ok(UserProfilePicture {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
