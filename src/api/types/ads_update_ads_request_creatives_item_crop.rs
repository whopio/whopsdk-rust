pub use crate::prelude::*;

/// The saved crop window for this creative, in source image pixels. Omit it for the original asset or for a format that has not been cropped.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAdsRequestCreativesItemCrop {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub y: Option<f64>,
}

impl UpdateAdsRequestCreativesItemCrop {
    pub fn builder() -> UpdateAdsRequestCreativesItemCropBuilder {
        <UpdateAdsRequestCreativesItemCropBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdsRequestCreativesItemCropBuilder {
    height: Option<f64>,
    width: Option<f64>,
    x: Option<f64>,
    y: Option<f64>,
}

impl UpdateAdsRequestCreativesItemCropBuilder {
    pub fn height(mut self, value: f64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn width(mut self, value: f64) -> Self {
        self.width = Some(value);
        self
    }

    pub fn x(mut self, value: f64) -> Self {
        self.x = Some(value);
        self
    }

    pub fn y(mut self, value: f64) -> Self {
        self.y = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdsRequestCreativesItemCrop`].
    pub fn build(self) -> Result<UpdateAdsRequestCreativesItemCrop, BuildError> {
        Ok(UpdateAdsRequestCreativesItemCrop {
            height: self.height,
            width: self.width,
            x: self.x,
            y: self.y,
        })
    }
}
