pub use crate::prelude::*;

/// The user's profile picture.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResendInviteBusinessesResponseBlueprintPartnerProfilePicture {
    /// The user's profile picture URL.
    #[serde(default)]
    pub url: String,
}

impl ResendInviteBusinessesResponseBlueprintPartnerProfilePicture {
    pub fn builder() -> ResendInviteBusinessesResponseBlueprintPartnerProfilePictureBuilder {
        <ResendInviteBusinessesResponseBlueprintPartnerProfilePictureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResendInviteBusinessesResponseBlueprintPartnerProfilePictureBuilder {
    url: Option<String>,
}

impl ResendInviteBusinessesResponseBlueprintPartnerProfilePictureBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResendInviteBusinessesResponseBlueprintPartnerProfilePicture`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](ResendInviteBusinessesResponseBlueprintPartnerProfilePictureBuilder::url)
    pub fn build(
        self,
    ) -> Result<ResendInviteBusinessesResponseBlueprintPartnerProfilePicture, BuildError> {
        Ok(
            ResendInviteBusinessesResponseBlueprintPartnerProfilePicture {
                url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            },
        )
    }
}
