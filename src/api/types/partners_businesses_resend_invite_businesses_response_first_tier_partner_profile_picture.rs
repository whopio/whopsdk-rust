pub use crate::prelude::*;

/// The user's profile picture.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResendInviteBusinessesResponseFirstTierPartnerProfilePicture {
    /// The user's profile picture URL.
    #[serde(default)]
    pub url: String,
}

impl ResendInviteBusinessesResponseFirstTierPartnerProfilePicture {
    pub fn builder() -> ResendInviteBusinessesResponseFirstTierPartnerProfilePictureBuilder {
        <ResendInviteBusinessesResponseFirstTierPartnerProfilePictureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResendInviteBusinessesResponseFirstTierPartnerProfilePictureBuilder {
    url: Option<String>,
}

impl ResendInviteBusinessesResponseFirstTierPartnerProfilePictureBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResendInviteBusinessesResponseFirstTierPartnerProfilePicture`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](ResendInviteBusinessesResponseFirstTierPartnerProfilePictureBuilder::url)
    pub fn build(
        self,
    ) -> Result<ResendInviteBusinessesResponseFirstTierPartnerProfilePicture, BuildError> {
        Ok(
            ResendInviteBusinessesResponseFirstTierPartnerProfilePicture {
                url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            },
        )
    }
}
