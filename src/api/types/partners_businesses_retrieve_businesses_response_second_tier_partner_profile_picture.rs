pub use crate::prelude::*;

/// The user's profile picture.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrieveBusinessesResponseSecondTierPartnerProfilePicture {
    /// The user's profile picture URL.
    #[serde(default)]
    pub url: String,
}

impl RetrieveBusinessesResponseSecondTierPartnerProfilePicture {
    pub fn builder() -> RetrieveBusinessesResponseSecondTierPartnerProfilePictureBuilder {
        <RetrieveBusinessesResponseSecondTierPartnerProfilePictureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveBusinessesResponseSecondTierPartnerProfilePictureBuilder {
    url: Option<String>,
}

impl RetrieveBusinessesResponseSecondTierPartnerProfilePictureBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrieveBusinessesResponseSecondTierPartnerProfilePicture`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](RetrieveBusinessesResponseSecondTierPartnerProfilePictureBuilder::url)
    pub fn build(
        self,
    ) -> Result<RetrieveBusinessesResponseSecondTierPartnerProfilePicture, BuildError> {
        Ok(RetrieveBusinessesResponseSecondTierPartnerProfilePicture {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
