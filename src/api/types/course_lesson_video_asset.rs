pub use crate::prelude::*;

/// The Mux video asset for video-type lessons, used for streaming playback. Null if this lesson has no hosted video.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CourseLessonVideoAsset {
    /// The Mux-provided ID of the asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    /// Whether this asset contains only audio
    #[serde(default)]
    pub audio_only: bool,
    /// The datetime the mux asset was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The duration of the video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,
    /// The time at which the video finished uploading
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub finished_uploading_at: Option<DateTime<FixedOffset>>,
    /// The unique identifier for the mux asset.
    #[serde(default)]
    pub id: String,
    /// The public playback ID of the Mux asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_id: Option<String>,
    /// The signed playback ID of the Mux asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_playback_id: Option<String>,
    /// The signed storyboard playback token of the Mux asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_storyboard_playback_token: Option<String>,
    /// The signed thumbnail playback token of the Mux asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_thumbnail_playback_token: Option<String>,
    /// The signed video playback token of the Mux asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_video_playback_token: Option<String>,
    /// The status of the Mux asset
    pub status: MuxAssetStatuses,
    /// The datetime the mux asset was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl CourseLessonVideoAsset {
    pub fn builder() -> CourseLessonVideoAssetBuilder {
        <CourseLessonVideoAssetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseLessonVideoAssetBuilder {
    asset_id: Option<String>,
    audio_only: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
    duration_seconds: Option<i64>,
    finished_uploading_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    playback_id: Option<String>,
    signed_playback_id: Option<String>,
    signed_storyboard_playback_token: Option<String>,
    signed_thumbnail_playback_token: Option<String>,
    signed_video_playback_token: Option<String>,
    status: Option<MuxAssetStatuses>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl CourseLessonVideoAssetBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn audio_only(mut self, value: bool) -> Self {
        self.audio_only = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn duration_seconds(mut self, value: i64) -> Self {
        self.duration_seconds = Some(value);
        self
    }

    pub fn finished_uploading_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.finished_uploading_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn playback_id(mut self, value: impl Into<String>) -> Self {
        self.playback_id = Some(value.into());
        self
    }

    pub fn signed_playback_id(mut self, value: impl Into<String>) -> Self {
        self.signed_playback_id = Some(value.into());
        self
    }

    pub fn signed_storyboard_playback_token(mut self, value: impl Into<String>) -> Self {
        self.signed_storyboard_playback_token = Some(value.into());
        self
    }

    pub fn signed_thumbnail_playback_token(mut self, value: impl Into<String>) -> Self {
        self.signed_thumbnail_playback_token = Some(value.into());
        self
    }

    pub fn signed_video_playback_token(mut self, value: impl Into<String>) -> Self {
        self.signed_video_playback_token = Some(value.into());
        self
    }

    pub fn status(mut self, value: MuxAssetStatuses) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CourseLessonVideoAsset`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio_only`](CourseLessonVideoAssetBuilder::audio_only)
    /// - [`created_at`](CourseLessonVideoAssetBuilder::created_at)
    /// - [`id`](CourseLessonVideoAssetBuilder::id)
    /// - [`status`](CourseLessonVideoAssetBuilder::status)
    /// - [`updated_at`](CourseLessonVideoAssetBuilder::updated_at)
    pub fn build(self) -> Result<CourseLessonVideoAsset, BuildError> {
        Ok(CourseLessonVideoAsset {
            asset_id: self.asset_id,
            audio_only: self
                .audio_only
                .ok_or_else(|| BuildError::missing_field("audio_only"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            duration_seconds: self.duration_seconds,
            finished_uploading_at: self.finished_uploading_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            playback_id: self.playback_id,
            signed_playback_id: self.signed_playback_id,
            signed_storyboard_playback_token: self.signed_storyboard_playback_token,
            signed_thumbnail_playback_token: self.signed_thumbnail_playback_token,
            signed_video_playback_token: self.signed_video_playback_token,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
