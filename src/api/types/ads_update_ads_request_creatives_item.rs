pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAdsRequestCreativesItem {
    /// The saved crop window for this creative, in source image pixels. Omit it for the original asset or for a format that has not been cropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop: Option<UpdateAdsRequestCreativesItemCrop>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<UpdateAdsRequestCreativesItemFormat>,
    /// Uploaded file ID, prefixed `file_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl UpdateAdsRequestCreativesItem {
    pub fn builder() -> UpdateAdsRequestCreativesItemBuilder {
        <UpdateAdsRequestCreativesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdsRequestCreativesItemBuilder {
    crop: Option<UpdateAdsRequestCreativesItemCrop>,
    format: Option<UpdateAdsRequestCreativesItemFormat>,
    id: Option<String>,
}

impl UpdateAdsRequestCreativesItemBuilder {
    pub fn crop(mut self, value: UpdateAdsRequestCreativesItemCrop) -> Self {
        self.crop = Some(value);
        self
    }

    pub fn format(mut self, value: UpdateAdsRequestCreativesItemFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdsRequestCreativesItem`].
    pub fn build(self) -> Result<UpdateAdsRequestCreativesItem, BuildError> {
        Ok(UpdateAdsRequestCreativesItem {
            crop: self.crop,
            format: self.format,
            id: self.id,
        })
    }
}
