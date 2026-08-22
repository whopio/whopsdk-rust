pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MediaAssetGeneration {
    /// Requested video length in seconds. `null` for images.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub duration_seconds: Option<f64>,
    /// What the asset was generated from.
    #[serde(default)]
    pub prompt: String,
    #[serde(default)]
    pub reference_media: Vec<String>,
    /// Requested video resolution. `null` for images. `1080p` is not supported by Seedance 2.0 Fast or Mini; `4k` is only supported by Seedance 2.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<MediaAssetGenerationResolution>,
}

impl MediaAssetGeneration {
    pub fn builder() -> MediaAssetGenerationBuilder {
        <MediaAssetGenerationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MediaAssetGenerationBuilder {
    duration_seconds: Option<f64>,
    prompt: Option<String>,
    reference_media: Option<Vec<String>>,
    resolution: Option<MediaAssetGenerationResolution>,
}

impl MediaAssetGenerationBuilder {
    pub fn duration_seconds(mut self, value: f64) -> Self {
        self.duration_seconds = Some(value);
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn reference_media(mut self, value: Vec<String>) -> Self {
        self.reference_media = Some(value);
        self
    }

    pub fn resolution(mut self, value: MediaAssetGenerationResolution) -> Self {
        self.resolution = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MediaAssetGeneration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](MediaAssetGenerationBuilder::prompt)
    /// - [`reference_media`](MediaAssetGenerationBuilder::reference_media)
    pub fn build(self) -> Result<MediaAssetGeneration, BuildError> {
        Ok(MediaAssetGeneration {
            duration_seconds: self.duration_seconds,
            prompt: self
                .prompt
                .ok_or_else(|| BuildError::missing_field("prompt"))?,
            reference_media: self
                .reference_media
                .ok_or_else(|| BuildError::missing_field("reference_media"))?,
            resolution: self.resolution,
        })
    }
}
