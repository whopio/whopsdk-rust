pub use crate::prelude::*;

/// The Mux video asset for video-type lessons, used for streaming playback. Null if this lesson has no hosted video.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseChaptersItemLessonsItemVideoAsset {
    /// The duration of the video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,
    /// The signed playback ID of the Mux asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_playback_id: Option<String>,
    /// The signed thumbnail playback token of the Mux asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_thumbnail_playback_token: Option<String>,
}

impl CourseChaptersItemLessonsItemVideoAsset {
    pub fn builder() -> CourseChaptersItemLessonsItemVideoAssetBuilder {
        <CourseChaptersItemLessonsItemVideoAssetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseChaptersItemLessonsItemVideoAssetBuilder {
    duration_seconds: Option<i64>,
    signed_playback_id: Option<String>,
    signed_thumbnail_playback_token: Option<String>,
}

impl CourseChaptersItemLessonsItemVideoAssetBuilder {
    pub fn duration_seconds(mut self, value: i64) -> Self {
        self.duration_seconds = Some(value);
        self
    }

    pub fn signed_playback_id(mut self, value: impl Into<String>) -> Self {
        self.signed_playback_id = Some(value.into());
        self
    }

    pub fn signed_thumbnail_playback_token(mut self, value: impl Into<String>) -> Self {
        self.signed_thumbnail_playback_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CourseChaptersItemLessonsItemVideoAsset`].
    pub fn build(self) -> Result<CourseChaptersItemLessonsItemVideoAsset, BuildError> {
        Ok(CourseChaptersItemLessonsItemVideoAsset {
            duration_seconds: self.duration_seconds,
            signed_playback_id: self.signed_playback_id,
            signed_thumbnail_playback_token: self.signed_thumbnail_playback_token,
        })
    }
}
