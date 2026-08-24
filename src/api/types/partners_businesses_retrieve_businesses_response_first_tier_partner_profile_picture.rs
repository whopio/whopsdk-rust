pub use crate::prelude::*;

/// The user's profile picture.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrieveBusinessesResponseFirstTierPartnerProfilePicture {
    /// The user's profile picture URL.
    #[serde(default)]
    pub url: String,
}

impl RetrieveBusinessesResponseFirstTierPartnerProfilePicture {
    pub fn builder() -> RetrieveBusinessesResponseFirstTierPartnerProfilePictureBuilder {
        <RetrieveBusinessesResponseFirstTierPartnerProfilePictureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveBusinessesResponseFirstTierPartnerProfilePictureBuilder {
    url: Option<String>,
}

impl RetrieveBusinessesResponseFirstTierPartnerProfilePictureBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrieveBusinessesResponseFirstTierPartnerProfilePicture`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](RetrieveBusinessesResponseFirstTierPartnerProfilePictureBuilder::url)
    pub fn build(
        self,
    ) -> Result<RetrieveBusinessesResponseFirstTierPartnerProfilePicture, BuildError> {
        Ok(RetrieveBusinessesResponseFirstTierPartnerProfilePicture {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
