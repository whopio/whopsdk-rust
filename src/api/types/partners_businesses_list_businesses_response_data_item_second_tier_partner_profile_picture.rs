pub use crate::prelude::*;

/// The user's profile picture.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBusinessesResponseDataItemSecondTierPartnerProfilePicture {
    /// The user's profile picture URL.
    #[serde(default)]
    pub url: String,
}

impl ListBusinessesResponseDataItemSecondTierPartnerProfilePicture {
    pub fn builder() -> ListBusinessesResponseDataItemSecondTierPartnerProfilePictureBuilder {
        <ListBusinessesResponseDataItemSecondTierPartnerProfilePictureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBusinessesResponseDataItemSecondTierPartnerProfilePictureBuilder {
    url: Option<String>,
}

impl ListBusinessesResponseDataItemSecondTierPartnerProfilePictureBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListBusinessesResponseDataItemSecondTierPartnerProfilePicture`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](ListBusinessesResponseDataItemSecondTierPartnerProfilePictureBuilder::url)
    pub fn build(
        self,
    ) -> Result<ListBusinessesResponseDataItemSecondTierPartnerProfilePicture, BuildError> {
        Ok(
            ListBusinessesResponseDataItemSecondTierPartnerProfilePicture {
                url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            },
        )
    }
}
