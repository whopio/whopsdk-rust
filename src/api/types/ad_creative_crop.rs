pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AdCreativeCrop {
    /// Height of the crop window in source pixels.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub height: f64,
    /// Width of the crop window in source pixels.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub width: f64,
    /// Left edge of the crop window in source pixels.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub x: f64,
    /// Top edge of the crop window in source pixels.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub y: f64,
}

impl AdCreativeCrop {
    pub fn builder() -> AdCreativeCropBuilder {
        <AdCreativeCropBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdCreativeCropBuilder {
    height: Option<f64>,
    width: Option<f64>,
    x: Option<f64>,
    y: Option<f64>,
}

impl AdCreativeCropBuilder {
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

    /// Consumes the builder and constructs a [`AdCreativeCrop`].
    /// This method will fail if any of the following fields are not set:
    /// - [`height`](AdCreativeCropBuilder::height)
    /// - [`width`](AdCreativeCropBuilder::width)
    /// - [`x`](AdCreativeCropBuilder::x)
    /// - [`y`](AdCreativeCropBuilder::y)
    pub fn build(self) -> Result<AdCreativeCrop, BuildError> {
        Ok(AdCreativeCrop {
            height: self
                .height
                .ok_or_else(|| BuildError::missing_field("height"))?,
            width: self
                .width
                .ok_or_else(|| BuildError::missing_field("width"))?,
            x: self.x.ok_or_else(|| BuildError::missing_field("x"))?,
            y: self.y.ok_or_else(|| BuildError::missing_field("y"))?,
        })
    }
}
