pub use crate::prelude::*;

/// The user's profile picture.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBusinessesResponseDataItemOwnerProfilePicture {
    /// The user's profile picture URL.
    #[serde(default)]
    pub url: String,
}

impl ListBusinessesResponseDataItemOwnerProfilePicture {
    pub fn builder() -> ListBusinessesResponseDataItemOwnerProfilePictureBuilder {
        <ListBusinessesResponseDataItemOwnerProfilePictureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBusinessesResponseDataItemOwnerProfilePictureBuilder {
    url: Option<String>,
}

impl ListBusinessesResponseDataItemOwnerProfilePictureBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListBusinessesResponseDataItemOwnerProfilePicture`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](ListBusinessesResponseDataItemOwnerProfilePictureBuilder::url)
    pub fn build(self) -> Result<ListBusinessesResponseDataItemOwnerProfilePicture, BuildError> {
        Ok(ListBusinessesResponseDataItemOwnerProfilePicture {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
