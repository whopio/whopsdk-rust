pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GenerateMediaRequest {
    /// Account ID, prefixed `biz_`. Defaults to the account the API key belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Video length in seconds. Video only; defaults to 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,
    /// What to generate. Up to 2,000 characters.
    #[serde(default)]
    pub prompt: String,
    /// Optional reference image file IDs (`file_` prefixed), up to 4. For video, a single reference seeds the opening frame; multiple references guide subject and style instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_media: Option<Vec<String>>,
    /// Video resolution. Video only; defaults to `1080p`. `1080p` is not supported by Seedance 2.0 Fast or Mini; `4k` is only supported by Seedance 2.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<GenerateMediaRequestResolution>,
    /// The kind of media to generate.
    pub r#type: GenerateMediaRequestType,
}

impl GenerateMediaRequest {
    pub fn builder() -> GenerateMediaRequestBuilder {
        <GenerateMediaRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GenerateMediaRequestBuilder {
    account_id: Option<String>,
    duration_seconds: Option<i64>,
    prompt: Option<String>,
    reference_media: Option<Vec<String>>,
    resolution: Option<GenerateMediaRequestResolution>,
    r#type: Option<GenerateMediaRequestType>,
}

impl GenerateMediaRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn duration_seconds(mut self, value: i64) -> Self {
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

    pub fn resolution(mut self, value: GenerateMediaRequestResolution) -> Self {
        self.resolution = Some(value);
        self
    }

    pub fn r#type(mut self, value: GenerateMediaRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GenerateMediaRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](GenerateMediaRequestBuilder::prompt)
    /// - [`r#type`](GenerateMediaRequestBuilder::r#type)
    pub fn build(self) -> Result<GenerateMediaRequest, BuildError> {
        Ok(GenerateMediaRequest {
            account_id: self.account_id,
            duration_seconds: self.duration_seconds,
            prompt: self
                .prompt
                .ok_or_else(|| BuildError::missing_field("prompt"))?,
            reference_media: self.reference_media,
            resolution: self.resolution,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
