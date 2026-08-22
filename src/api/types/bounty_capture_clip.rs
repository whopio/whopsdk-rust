pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BountyCaptureClip {
    /// The bounty submission (attempt) this clip belongs to, prefixed `btys_`.
    #[serde(default)]
    pub bounty_submission_id: String,
    /// When the clip was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Server-validated clip duration in whole seconds. `null` until validation completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,
    /// Stable validation failure code. `null` unless `status` is `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// Human-readable validation failure reason. `null` unless `status` is `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    /// Temporary signed URL for the video frame timestamp log. Returned only on single-clip reads for an authorized viewer; `null` on list responses or until the artifact is attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frames_url: Option<String>,
    /// Capture clip ID, prefixed `bclip_`.
    #[serde(default)]
    pub id: String,
    /// Temporary signed URL for the IMU (accelerometer + gyroscope) log. Returned only on single-clip reads for an authorized viewer; `null` on list responses or until the artifact is attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imu_url: Option<String>,
    /// Temporary signed URL for the capture manifest. Returned only on single-clip reads for an authorized viewer; `null` on list responses or until the artifact is attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_url: Option<String>,
    /// When server-side validation completed successfully, as an ISO 8601 timestamp. `null` until then.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_at: Option<String>,
    /// The clip's stable order within the attempt, starting at 1.
    #[serde(default)]
    pub sequence: i64,
    /// Recording and validation state. `recording` is still capturing; `verifying` is running server-side validation; `ready` passed validation and counts toward the verified-duration payout gate; `failed` did not validate.
    pub status: BountyCaptureClipStatus,
    /// When the clip was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// Temporary signed URL for the synchronized MP4 video. Returned only on single-clip reads for an authorized viewer; `null` on list responses or until the artifact is attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_url: Option<String>,
}

impl BountyCaptureClip {
    pub fn builder() -> BountyCaptureClipBuilder {
        <BountyCaptureClipBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BountyCaptureClipBuilder {
    bounty_submission_id: Option<String>,
    created_at: Option<String>,
    duration_seconds: Option<i64>,
    failure_code: Option<String>,
    failure_message: Option<String>,
    frames_url: Option<String>,
    id: Option<String>,
    imu_url: Option<String>,
    manifest_url: Option<String>,
    ready_at: Option<String>,
    sequence: Option<i64>,
    status: Option<BountyCaptureClipStatus>,
    updated_at: Option<String>,
    video_url: Option<String>,
}

impl BountyCaptureClipBuilder {
    pub fn bounty_submission_id(mut self, value: impl Into<String>) -> Self {
        self.bounty_submission_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn duration_seconds(mut self, value: i64) -> Self {
        self.duration_seconds = Some(value);
        self
    }

    pub fn failure_code(mut self, value: impl Into<String>) -> Self {
        self.failure_code = Some(value.into());
        self
    }

    pub fn failure_message(mut self, value: impl Into<String>) -> Self {
        self.failure_message = Some(value.into());
        self
    }

    pub fn frames_url(mut self, value: impl Into<String>) -> Self {
        self.frames_url = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn imu_url(mut self, value: impl Into<String>) -> Self {
        self.imu_url = Some(value.into());
        self
    }

    pub fn manifest_url(mut self, value: impl Into<String>) -> Self {
        self.manifest_url = Some(value.into());
        self
    }

    pub fn ready_at(mut self, value: impl Into<String>) -> Self {
        self.ready_at = Some(value.into());
        self
    }

    pub fn sequence(mut self, value: i64) -> Self {
        self.sequence = Some(value);
        self
    }

    pub fn status(mut self, value: BountyCaptureClipStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn video_url(mut self, value: impl Into<String>) -> Self {
        self.video_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BountyCaptureClip`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bounty_submission_id`](BountyCaptureClipBuilder::bounty_submission_id)
    /// - [`created_at`](BountyCaptureClipBuilder::created_at)
    /// - [`id`](BountyCaptureClipBuilder::id)
    /// - [`sequence`](BountyCaptureClipBuilder::sequence)
    /// - [`status`](BountyCaptureClipBuilder::status)
    /// - [`updated_at`](BountyCaptureClipBuilder::updated_at)
    pub fn build(self) -> Result<BountyCaptureClip, BuildError> {
        Ok(BountyCaptureClip {
            bounty_submission_id: self
                .bounty_submission_id
                .ok_or_else(|| BuildError::missing_field("bounty_submission_id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            duration_seconds: self.duration_seconds,
            failure_code: self.failure_code,
            failure_message: self.failure_message,
            frames_url: self.frames_url,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            imu_url: self.imu_url,
            manifest_url: self.manifest_url,
            ready_at: self.ready_at,
            sequence: self
                .sequence
                .ok_or_else(|| BuildError::missing_field("sequence"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            video_url: self.video_url,
        })
    }
}
