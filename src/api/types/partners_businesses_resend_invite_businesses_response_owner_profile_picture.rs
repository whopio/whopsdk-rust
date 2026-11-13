pub use crate::prelude::*;

/// The user's profile picture.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResendInviteBusinessesResponseOwnerProfilePicture {
    /// The user's profile picture URL.
    #[serde(default)]
    pub url: String,
}

impl ResendInviteBusinessesResponseOwnerProfilePicture {
    pub fn builder() -> ResendInviteBusinessesResponseOwnerProfilePictureBuilder {
        <ResendInviteBusinessesResponseOwnerProfilePictureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResendInviteBusinessesResponseOwnerProfilePictureBuilder {
    url: Option<String>,
}

impl ResendInviteBusinessesResponseOwnerProfilePictureBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResendInviteBusinessesResponseOwnerProfilePicture`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](ResendInviteBusinessesResponseOwnerProfilePictureBuilder::url)
    pub fn build(self) -> Result<ResendInviteBusinessesResponseOwnerProfilePicture, BuildError> {
        Ok(ResendInviteBusinessesResponseOwnerProfilePicture {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
