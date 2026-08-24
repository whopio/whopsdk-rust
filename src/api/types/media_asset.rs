pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MediaAsset {
    /// USD amount charged to the account's balance for this generation. `null` if the generation wasn't billed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount_charged: Option<f64>,
    /// ISO 8601 timestamp when the asset reached a terminal state. `null` while `processing`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// ISO 8601 timestamp when the generation was requested.
    #[serde(default)]
    pub created_at: String,
    /// Currency of `amount_charged`. Always `usd`.
    #[serde(default)]
    pub currency: String,
    /// Why generation failed. `null` unless status is `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// The produced file, usable anywhere attachments are accepted. `null` until the asset is `ready`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<MediaAssetFile>,
    /// The inputs the asset was generated from.
    #[serde(default)]
    pub generation: MediaAssetGeneration,
    /// Media asset ID, prefixed `media_`.
    #[serde(default)]
    pub id: String,
    /// The kind of media this asset holds.
    pub media_type: MediaAssetMediaType,
    /// How the asset was created. Always `generated`.
    pub source: MediaAssetSource,
    /// Lifecycle state: `processing` while generation runs, `ready` when the file is available, `failed` when generation failed and the charge was refunded.
    pub status: MediaAssetStatus,
}

impl MediaAsset {
    pub fn builder() -> MediaAssetBuilder {
        <MediaAssetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MediaAssetBuilder {
    amount_charged: Option<f64>,
    completed_at: Option<String>,
    created_at: Option<String>,
    currency: Option<String>,
    error_message: Option<String>,
    file: Option<MediaAssetFile>,
    generation: Option<MediaAssetGeneration>,
    id: Option<String>,
    media_type: Option<MediaAssetMediaType>,
    source: Option<MediaAssetSource>,
    status: Option<MediaAssetStatus>,
}

impl MediaAssetBuilder {
    pub fn amount_charged(mut self, value: f64) -> Self {
        self.amount_charged = Some(value);
        self
    }

    pub fn completed_at(mut self, value: impl Into<String>) -> Self {
        self.completed_at = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn file(mut self, value: MediaAssetFile) -> Self {
        self.file = Some(value);
        self
    }

    pub fn generation(mut self, value: MediaAssetGeneration) -> Self {
        self.generation = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn media_type(mut self, value: MediaAssetMediaType) -> Self {
        self.media_type = Some(value);
        self
    }

    pub fn source(mut self, value: MediaAssetSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn status(mut self, value: MediaAssetStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MediaAsset`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](MediaAssetBuilder::created_at)
    /// - [`currency`](MediaAssetBuilder::currency)
    /// - [`generation`](MediaAssetBuilder::generation)
    /// - [`id`](MediaAssetBuilder::id)
    /// - [`media_type`](MediaAssetBuilder::media_type)
    /// - [`source`](MediaAssetBuilder::source)
    /// - [`status`](MediaAssetBuilder::status)
    pub fn build(self) -> Result<MediaAsset, BuildError> {
        Ok(MediaAsset {
            amount_charged: self.amount_charged,
            completed_at: self.completed_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            error_message: self.error_message,
            file: self.file,
            generation: self
                .generation
                .ok_or_else(|| BuildError::missing_field("generation"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            media_type: self
                .media_type
                .ok_or_else(|| BuildError::missing_field("media_type"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
