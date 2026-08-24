pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BountySubmissionLivestreamFeed {
    /// When the proof livestream ended, as an ISO 8601 timestamp. `null` while it is still live — a feed with a `started_at` and no `ended_at` is streaming right now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    /// Livestream feed ID.
    #[serde(default)]
    pub id: String,
    /// Recording lifecycle state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_status: Option<BountySubmissionLivestreamFeedRecordingStatus>,
    /// Playback URL for a completed proof recording, when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_url: Option<String>,
    /// When the proof livestream went live, as an ISO 8601 timestamp. `null` before it starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// Current proof thumbnail URL, when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// Display title for the proof livestream.
    #[serde(default)]
    pub title: String,
}

impl BountySubmissionLivestreamFeed {
    pub fn builder() -> BountySubmissionLivestreamFeedBuilder {
        <BountySubmissionLivestreamFeedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BountySubmissionLivestreamFeedBuilder {
    ended_at: Option<String>,
    id: Option<String>,
    recording_status: Option<BountySubmissionLivestreamFeedRecordingStatus>,
    recording_url: Option<String>,
    started_at: Option<String>,
    thumbnail_url: Option<String>,
    title: Option<String>,
}

impl BountySubmissionLivestreamFeedBuilder {
    pub fn ended_at(mut self, value: impl Into<String>) -> Self {
        self.ended_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn recording_status(
        mut self,
        value: BountySubmissionLivestreamFeedRecordingStatus,
    ) -> Self {
        self.recording_status = Some(value);
        self
    }

    pub fn recording_url(mut self, value: impl Into<String>) -> Self {
        self.recording_url = Some(value.into());
        self
    }

    pub fn started_at(mut self, value: impl Into<String>) -> Self {
        self.started_at = Some(value.into());
        self
    }

    pub fn thumbnail_url(mut self, value: impl Into<String>) -> Self {
        self.thumbnail_url = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BountySubmissionLivestreamFeed`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BountySubmissionLivestreamFeedBuilder::id)
    /// - [`title`](BountySubmissionLivestreamFeedBuilder::title)
    pub fn build(self) -> Result<BountySubmissionLivestreamFeed, BuildError> {
        Ok(BountySubmissionLivestreamFeed {
            ended_at: self.ended_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            recording_status: self.recording_status,
            recording_url: self.recording_url,
            started_at: self.started_at,
            thumbnail_url: self.thumbnail_url,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
