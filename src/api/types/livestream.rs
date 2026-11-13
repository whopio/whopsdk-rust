pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Livestream {
    /// When the livestream was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// When the livestream ended, as an ISO 8601 timestamp. `null` while it is live or before it starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    /// Livestream ID, prefixed `lfeed_`.
    #[serde(default)]
    pub id: String,
    /// When a moderator pinned this livestream's recording, as an ISO 8601 timestamp. `null` when it is not pinned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_at: Option<String>,
    /// When the recording's first frame was captured, as an ISO 8601 timestamp. Offsets into the recording count from here, not from `started_at`. `null` until recording starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_started_at: Option<String>,
    /// State of the recording. `null` when the livestream was not recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_status: Option<LivestreamRecordingStatus>,
    /// A temporary link to the recording's video file. Retrieve the livestream again for a fresh link. `null` until the recording is ready, or when it was deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_url: Option<String>,
    /// When the livestream went live, as an ISO 8601 timestamp. `null` before it starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// A temporary link to a still frame from the livestream. `null` when none was captured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// Display title of the livestream. `Untitled Livestream` when the host did not set one.
    #[serde(default)]
    pub title: String,
}

impl Livestream {
    pub fn builder() -> LivestreamBuilder {
        <LivestreamBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LivestreamBuilder {
    created_at: Option<String>,
    ended_at: Option<String>,
    id: Option<String>,
    pinned_at: Option<String>,
    recording_started_at: Option<String>,
    recording_status: Option<LivestreamRecordingStatus>,
    recording_url: Option<String>,
    started_at: Option<String>,
    thumbnail_url: Option<String>,
    title: Option<String>,
}

impl LivestreamBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn ended_at(mut self, value: impl Into<String>) -> Self {
        self.ended_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn pinned_at(mut self, value: impl Into<String>) -> Self {
        self.pinned_at = Some(value.into());
        self
    }

    pub fn recording_started_at(mut self, value: impl Into<String>) -> Self {
        self.recording_started_at = Some(value.into());
        self
    }

    pub fn recording_status(mut self, value: LivestreamRecordingStatus) -> Self {
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

    /// Consumes the builder and constructs a [`Livestream`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](LivestreamBuilder::created_at)
    /// - [`id`](LivestreamBuilder::id)
    /// - [`title`](LivestreamBuilder::title)
    pub fn build(self) -> Result<Livestream, BuildError> {
        Ok(Livestream {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            ended_at: self.ended_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            pinned_at: self.pinned_at,
            recording_started_at: self.recording_started_at,
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
