pub use crate::prelude::*;

/// The user's profile picture.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResendInviteBusinessesResponseSecondTierPartnerProfilePicture {
    /// The user's profile picture URL.
    #[serde(default)]
    pub url: String,
}

impl ResendInviteBusinessesResponseSecondTierPartnerProfilePicture {
    pub fn builder() -> ResendInviteBusinessesResponseSecondTierPartnerProfilePictureBuilder {
        <ResendInviteBusinessesResponseSecondTierPartnerProfilePictureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResendInviteBusinessesResponseSecondTierPartnerProfilePictureBuilder {
    url: Option<String>,
}

impl ResendInviteBusinessesResponseSecondTierPartnerProfilePictureBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResendInviteBusinessesResponseSecondTierPartnerProfilePicture`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](ResendInviteBusinessesResponseSecondTierPartnerProfilePictureBuilder::url)
    pub fn build(
        self,
    ) -> Result<ResendInviteBusinessesResponseSecondTierPartnerProfilePicture, BuildError> {
        Ok(
            ResendInviteBusinessesResponseSecondTierPartnerProfilePicture {
                url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            },
        )
    }
}
