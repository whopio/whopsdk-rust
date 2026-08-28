pub use crate::prelude::*;

/// The user's profile picture.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBusinessesResponseDataItemBlueprintPartnerProfilePicture {
    /// The user's profile picture URL.
    #[serde(default)]
    pub url: String,
}

impl ListBusinessesResponseDataItemBlueprintPartnerProfilePicture {
    pub fn builder() -> ListBusinessesResponseDataItemBlueprintPartnerProfilePictureBuilder {
        <ListBusinessesResponseDataItemBlueprintPartnerProfilePictureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBusinessesResponseDataItemBlueprintPartnerProfilePictureBuilder {
    url: Option<String>,
}

impl ListBusinessesResponseDataItemBlueprintPartnerProfilePictureBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListBusinessesResponseDataItemBlueprintPartnerProfilePicture`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](ListBusinessesResponseDataItemBlueprintPartnerProfilePictureBuilder::url)
    pub fn build(
        self,
    ) -> Result<ListBusinessesResponseDataItemBlueprintPartnerProfilePicture, BuildError> {
        Ok(
            ListBusinessesResponseDataItemBlueprintPartnerProfilePicture {
                url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            },
        )
    }
}
