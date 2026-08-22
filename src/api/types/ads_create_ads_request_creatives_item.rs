pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateAdsRequestCreativesItem {
    /// The saved crop window for this creative, in source image pixels. Omit it for the original asset or for a format that has not been cropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop: Option<CreateAdsRequestCreativesItemCrop>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CreateAdsRequestCreativesItemFormat>,
    /// Uploaded file ID, prefixed `file_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl CreateAdsRequestCreativesItem {
    pub fn builder() -> CreateAdsRequestCreativesItemBuilder {
        <CreateAdsRequestCreativesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdsRequestCreativesItemBuilder {
    crop: Option<CreateAdsRequestCreativesItemCrop>,
    format: Option<CreateAdsRequestCreativesItemFormat>,
    id: Option<String>,
}

impl CreateAdsRequestCreativesItemBuilder {
    pub fn crop(mut self, value: CreateAdsRequestCreativesItemCrop) -> Self {
        self.crop = Some(value);
        self
    }

    pub fn format(mut self, value: CreateAdsRequestCreativesItemFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAdsRequestCreativesItem`].
    pub fn build(self) -> Result<CreateAdsRequestCreativesItem, BuildError> {
        Ok(CreateAdsRequestCreativesItem {
            crop: self.crop,
            format: self.format,
            id: self.id,
        })
    }
}
