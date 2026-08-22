pub use crate::prelude::*;

/// The user's profile picture.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBusinessesResponseDataItemFirstTierPartnerProfilePicture {
    /// The user's profile picture URL.
    #[serde(default)]
    pub url: String,
}

impl ListBusinessesResponseDataItemFirstTierPartnerProfilePicture {
    pub fn builder() -> ListBusinessesResponseDataItemFirstTierPartnerProfilePictureBuilder {
        <ListBusinessesResponseDataItemFirstTierPartnerProfilePictureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBusinessesResponseDataItemFirstTierPartnerProfilePictureBuilder {
    url: Option<String>,
}

impl ListBusinessesResponseDataItemFirstTierPartnerProfilePictureBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListBusinessesResponseDataItemFirstTierPartnerProfilePicture`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](ListBusinessesResponseDataItemFirstTierPartnerProfilePictureBuilder::url)
    pub fn build(
        self,
    ) -> Result<ListBusinessesResponseDataItemFirstTierPartnerProfilePicture, BuildError> {
        Ok(
            ListBusinessesResponseDataItemFirstTierPartnerProfilePicture {
                url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            },
        )
    }
}
