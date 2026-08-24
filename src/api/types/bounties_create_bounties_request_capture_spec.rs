pub use crate::prelude::*;

/// Per-bounty overrides of the served capture contract. Only accepted when `business_goal_type` is `data_capture`; omitted fields keep the platform defaults, and the resulting contract is echoed back as `capture_spec` on the bounty.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBountiesRequestCaptureSpec {
    /// Average bitrate the recorder encodes at, in megabits per second. Must sit within the served floor and ceiling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate_target_mbps: Option<i64>,
    /// Whether the recorder also writes camera make and model into the video container's metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_camera_metadata: Option<bool>,
    /// Longest stall between consecutive frames a clip may contain before the client rejects it, in milliseconds. Unlike the recording fields this one can also be tuned after the bounty is created, since it bounds what is accepted rather than how footage is captured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_gap_tolerance_ms: Option<i64>,
    /// Minimum length of a single clip, in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_clip_duration_seconds: Option<i64>,
    /// Total verified footage a submission must accumulate across all its clips before it can be submitted, in seconds. Must be a whole number of hours between 1 and 12. Editable after create, until someone starts an attempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_total_verified_duration_seconds: Option<i64>,
    /// How the recorder configures video stabilization. `off` preserves raw motion for pose extraction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stabilization_mode: Option<CreateBountiesRequestCaptureSpecStabilizationMode>,
}

impl CreateBountiesRequestCaptureSpec {
    pub fn builder() -> CreateBountiesRequestCaptureSpecBuilder {
        <CreateBountiesRequestCaptureSpecBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBountiesRequestCaptureSpecBuilder {
    bitrate_target_mbps: Option<i64>,
    embed_camera_metadata: Option<bool>,
    frame_gap_tolerance_ms: Option<i64>,
    min_clip_duration_seconds: Option<i64>,
    min_total_verified_duration_seconds: Option<i64>,
    stabilization_mode: Option<CreateBountiesRequestCaptureSpecStabilizationMode>,
}

impl CreateBountiesRequestCaptureSpecBuilder {
    pub fn bitrate_target_mbps(mut self, value: i64) -> Self {
        self.bitrate_target_mbps = Some(value);
        self
    }

    pub fn embed_camera_metadata(mut self, value: bool) -> Self {
        self.embed_camera_metadata = Some(value);
        self
    }

    pub fn frame_gap_tolerance_ms(mut self, value: i64) -> Self {
        self.frame_gap_tolerance_ms = Some(value);
        self
    }

    pub fn min_clip_duration_seconds(mut self, value: i64) -> Self {
        self.min_clip_duration_seconds = Some(value);
        self
    }

    pub fn min_total_verified_duration_seconds(mut self, value: i64) -> Self {
        self.min_total_verified_duration_seconds = Some(value);
        self
    }

    pub fn stabilization_mode(
        mut self,
        value: CreateBountiesRequestCaptureSpecStabilizationMode,
    ) -> Self {
        self.stabilization_mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateBountiesRequestCaptureSpec`].
    pub fn build(self) -> Result<CreateBountiesRequestCaptureSpec, BuildError> {
        Ok(CreateBountiesRequestCaptureSpec {
            bitrate_target_mbps: self.bitrate_target_mbps,
            embed_camera_metadata: self.embed_camera_metadata,
            frame_gap_tolerance_ms: self.frame_gap_tolerance_ms,
            min_clip_duration_seconds: self.min_clip_duration_seconds,
            min_total_verified_duration_seconds: self.min_total_verified_duration_seconds,
            stabilization_mode: self.stabilization_mode,
        })
    }
}
