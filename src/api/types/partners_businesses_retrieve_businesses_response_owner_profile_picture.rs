pub use crate::prelude::*;

/// The user's profile picture.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrieveBusinessesResponseOwnerProfilePicture {
    /// The user's profile picture URL.
    #[serde(default)]
    pub url: String,
}

impl RetrieveBusinessesResponseOwnerProfilePicture {
    pub fn builder() -> RetrieveBusinessesResponseOwnerProfilePictureBuilder {
        <RetrieveBusinessesResponseOwnerProfilePictureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveBusinessesResponseOwnerProfilePictureBuilder {
    url: Option<String>,
}

impl RetrieveBusinessesResponseOwnerProfilePictureBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrieveBusinessesResponseOwnerProfilePicture`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](RetrieveBusinessesResponseOwnerProfilePictureBuilder::url)
    pub fn build(self) -> Result<RetrieveBusinessesResponseOwnerProfilePicture, BuildError> {
        Ok(RetrieveBusinessesResponseOwnerProfilePicture {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
