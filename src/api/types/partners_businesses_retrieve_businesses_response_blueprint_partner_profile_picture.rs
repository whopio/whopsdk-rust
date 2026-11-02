pub use crate::prelude::*;

/// The user's profile picture.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrieveBusinessesResponseBlueprintPartnerProfilePicture {
    /// The user's profile picture URL.
    #[serde(default)]
    pub url: String,
}

impl RetrieveBusinessesResponseBlueprintPartnerProfilePicture {
    pub fn builder() -> RetrieveBusinessesResponseBlueprintPartnerProfilePictureBuilder {
        <RetrieveBusinessesResponseBlueprintPartnerProfilePictureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveBusinessesResponseBlueprintPartnerProfilePictureBuilder {
    url: Option<String>,
}

impl RetrieveBusinessesResponseBlueprintPartnerProfilePictureBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrieveBusinessesResponseBlueprintPartnerProfilePicture`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](RetrieveBusinessesResponseBlueprintPartnerProfilePictureBuilder::url)
    pub fn build(
        self,
    ) -> Result<RetrieveBusinessesResponseBlueprintPartnerProfilePicture, BuildError> {
        Ok(RetrieveBusinessesResponseBlueprintPartnerProfilePicture {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
