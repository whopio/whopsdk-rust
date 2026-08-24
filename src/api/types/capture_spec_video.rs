pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CaptureSpecVideo {
    /// Maximum acceptable average bitrate, in megabits per second.
    #[serde(default)]
    pub bitrate_ceiling_mbps: i64,
    /// Minimum acceptable average bitrate, in megabits per second.
    #[serde(default)]
    pub bitrate_floor_mbps: i64,
    /// Recommended average bitrate to encode at, in megabits per second.
    #[serde(default)]
    pub bitrate_target_mbps: i64,
    /// Which physical lens to record with.
    #[serde(default)]
    pub camera_lens: String,
    #[serde(default)]
    pub codecs: Vec<String>,
    /// Whether the client must also write the camera make and model into the video container's metadata. When `false`, the capture manifest and export CSV are the metadata carrier.
    #[serde(default)]
    pub embed_camera_metadata: bool,
    /// Target capture frame rate.
    #[serde(default)]
    pub fps: i64,
    /// Longest stall between consecutive frames a clip may contain before the client rejects it, in milliseconds. Every frame is timestamped in the frame log, so a stall stays alignable downstream — this bounds how broken a capture may be, not how evenly it must be paced.
    #[serde(default)]
    pub frame_gap_tolerance_ms: i64,
    /// Required frame height in pixels — recorded footage must match exactly.
    #[serde(default)]
    pub height: i64,
    /// Minimum acceptable horizontal field of view, in degrees.
    #[serde(default)]
    pub min_fov_degrees: i64,
    /// Device orientation to record in.
    #[serde(default)]
    pub orientation: String,
    /// Preferred horizontal field of view, in degrees.
    #[serde(default)]
    pub preferred_fov_degrees: i64,
    /// How the client must configure video stabilization: `off` disables EIS so raw motion is preserved for pose extraction, `on` requires it, `any` leaves the device default.
    pub stabilization_mode: CaptureSpecVideoStabilizationMode,
    /// Whether hardware/software stabilization must be enabled. True exactly when stabilization_mode is `on`.
    #[serde(default)]
    pub stabilization_required: bool,
    /// Required frame width in pixels — recorded footage must match exactly.
    #[serde(default)]
    pub width: i64,
}

impl CaptureSpecVideo {
    pub fn builder() -> CaptureSpecVideoBuilder {
        <CaptureSpecVideoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptureSpecVideoBuilder {
    bitrate_ceiling_mbps: Option<i64>,
    bitrate_floor_mbps: Option<i64>,
    bitrate_target_mbps: Option<i64>,
    camera_lens: Option<String>,
    codecs: Option<Vec<String>>,
    embed_camera_metadata: Option<bool>,
    fps: Option<i64>,
    frame_gap_tolerance_ms: Option<i64>,
    height: Option<i64>,
    min_fov_degrees: Option<i64>,
    orientation: Option<String>,
    preferred_fov_degrees: Option<i64>,
    stabilization_mode: Option<CaptureSpecVideoStabilizationMode>,
    stabilization_required: Option<bool>,
    width: Option<i64>,
}

impl CaptureSpecVideoBuilder {
    pub fn bitrate_ceiling_mbps(mut self, value: i64) -> Self {
        self.bitrate_ceiling_mbps = Some(value);
        self
    }

    pub fn bitrate_floor_mbps(mut self, value: i64) -> Self {
        self.bitrate_floor_mbps = Some(value);
        self
    }

    pub fn bitrate_target_mbps(mut self, value: i64) -> Self {
        self.bitrate_target_mbps = Some(value);
        self
    }

    pub fn camera_lens(mut self, value: impl Into<String>) -> Self {
        self.camera_lens = Some(value.into());
        self
    }

    pub fn codecs(mut self, value: Vec<String>) -> Self {
        self.codecs = Some(value);
        self
    }

    pub fn embed_camera_metadata(mut self, value: bool) -> Self {
        self.embed_camera_metadata = Some(value);
        self
    }

    pub fn fps(mut self, value: i64) -> Self {
        self.fps = Some(value);
        self
    }

    pub fn frame_gap_tolerance_ms(mut self, value: i64) -> Self {
        self.frame_gap_tolerance_ms = Some(value);
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn min_fov_degrees(mut self, value: i64) -> Self {
        self.min_fov_degrees = Some(value);
        self
    }

    pub fn orientation(mut self, value: impl Into<String>) -> Self {
        self.orientation = Some(value.into());
        self
    }

    pub fn preferred_fov_degrees(mut self, value: i64) -> Self {
        self.preferred_fov_degrees = Some(value);
        self
    }

    pub fn stabilization_mode(mut self, value: CaptureSpecVideoStabilizationMode) -> Self {
        self.stabilization_mode = Some(value);
        self
    }

    pub fn stabilization_required(mut self, value: bool) -> Self {
        self.stabilization_required = Some(value);
        self
    }

    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptureSpecVideo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bitrate_ceiling_mbps`](CaptureSpecVideoBuilder::bitrate_ceiling_mbps)
    /// - [`bitrate_floor_mbps`](CaptureSpecVideoBuilder::bitrate_floor_mbps)
    /// - [`bitrate_target_mbps`](CaptureSpecVideoBuilder::bitrate_target_mbps)
    /// - [`camera_lens`](CaptureSpecVideoBuilder::camera_lens)
    /// - [`codecs`](CaptureSpecVideoBuilder::codecs)
    /// - [`embed_camera_metadata`](CaptureSpecVideoBuilder::embed_camera_metadata)
    /// - [`fps`](CaptureSpecVideoBuilder::fps)
    /// - [`frame_gap_tolerance_ms`](CaptureSpecVideoBuilder::frame_gap_tolerance_ms)
    /// - [`height`](CaptureSpecVideoBuilder::height)
    /// - [`min_fov_degrees`](CaptureSpecVideoBuilder::min_fov_degrees)
    /// - [`orientation`](CaptureSpecVideoBuilder::orientation)
    /// - [`preferred_fov_degrees`](CaptureSpecVideoBuilder::preferred_fov_degrees)
    /// - [`stabilization_mode`](CaptureSpecVideoBuilder::stabilization_mode)
    /// - [`stabilization_required`](CaptureSpecVideoBuilder::stabilization_required)
    /// - [`width`](CaptureSpecVideoBuilder::width)
    pub fn build(self) -> Result<CaptureSpecVideo, BuildError> {
        Ok(CaptureSpecVideo {
            bitrate_ceiling_mbps: self
                .bitrate_ceiling_mbps
                .ok_or_else(|| BuildError::missing_field("bitrate_ceiling_mbps"))?,
            bitrate_floor_mbps: self
                .bitrate_floor_mbps
                .ok_or_else(|| BuildError::missing_field("bitrate_floor_mbps"))?,
            bitrate_target_mbps: self
                .bitrate_target_mbps
                .ok_or_else(|| BuildError::missing_field("bitrate_target_mbps"))?,
            camera_lens: self
                .camera_lens
                .ok_or_else(|| BuildError::missing_field("camera_lens"))?,
            codecs: self
                .codecs
                .ok_or_else(|| BuildError::missing_field("codecs"))?,
            embed_camera_metadata: self
                .embed_camera_metadata
                .ok_or_else(|| BuildError::missing_field("embed_camera_metadata"))?,
            fps: self.fps.ok_or_else(|| BuildError::missing_field("fps"))?,
            frame_gap_tolerance_ms: self
                .frame_gap_tolerance_ms
                .ok_or_else(|| BuildError::missing_field("frame_gap_tolerance_ms"))?,
            height: self
                .height
                .ok_or_else(|| BuildError::missing_field("height"))?,
            min_fov_degrees: self
                .min_fov_degrees
                .ok_or_else(|| BuildError::missing_field("min_fov_degrees"))?,
            orientation: self
                .orientation
                .ok_or_else(|| BuildError::missing_field("orientation"))?,
            preferred_fov_degrees: self
                .preferred_fov_degrees
                .ok_or_else(|| BuildError::missing_field("preferred_fov_degrees"))?,
            stabilization_mode: self
                .stabilization_mode
                .ok_or_else(|| BuildError::missing_field("stabilization_mode"))?,
            stabilization_required: self
                .stabilization_required
                .ok_or_else(|| BuildError::missing_field("stabilization_required"))?,
            width: self
                .width
                .ok_or_else(|| BuildError::missing_field("width"))?,
        })
    }
}
