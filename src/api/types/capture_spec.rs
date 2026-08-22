pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CaptureSpec {
    /// The naming convention for uploaded files, built from the required metadata fields.
    #[serde(default)]
    pub filename_pattern: String,
    /// Inertial measurement unit (IMU) recording requirements.
    #[serde(default)]
    pub imu: CaptureSpecImu,
    /// Schema version the client must stamp on the capture manifest it uploads.
    #[serde(default)]
    pub manifest_schema_version: i64,
    /// Minimum length of a single clip, in seconds.
    #[serde(default)]
    pub min_clip_duration_seconds: i64,
    /// Total verified footage a submission must accumulate across all its clips before it can be submitted, in seconds. Always a whole number of hours.
    #[serde(default)]
    pub min_total_verified_duration_seconds: i64,
    #[serde(default)]
    pub required_metadata_fields: Vec<String>,
    /// Whether each clip must be one uninterrupted recording rather than stitched segments.
    #[serde(default)]
    pub single_continuous_take: bool,
    /// Video recording requirements.
    pub video: CaptureSpecVideo,
}

impl CaptureSpec {
    pub fn builder() -> CaptureSpecBuilder {
        <CaptureSpecBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptureSpecBuilder {
    filename_pattern: Option<String>,
    imu: Option<CaptureSpecImu>,
    manifest_schema_version: Option<i64>,
    min_clip_duration_seconds: Option<i64>,
    min_total_verified_duration_seconds: Option<i64>,
    required_metadata_fields: Option<Vec<String>>,
    single_continuous_take: Option<bool>,
    video: Option<CaptureSpecVideo>,
}

impl CaptureSpecBuilder {
    pub fn filename_pattern(mut self, value: impl Into<String>) -> Self {
        self.filename_pattern = Some(value.into());
        self
    }

    pub fn imu(mut self, value: CaptureSpecImu) -> Self {
        self.imu = Some(value);
        self
    }

    pub fn manifest_schema_version(mut self, value: i64) -> Self {
        self.manifest_schema_version = Some(value);
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

    pub fn required_metadata_fields(mut self, value: Vec<String>) -> Self {
        self.required_metadata_fields = Some(value);
        self
    }

    pub fn single_continuous_take(mut self, value: bool) -> Self {
        self.single_continuous_take = Some(value);
        self
    }

    pub fn video(mut self, value: CaptureSpecVideo) -> Self {
        self.video = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptureSpec`].
    /// This method will fail if any of the following fields are not set:
    /// - [`filename_pattern`](CaptureSpecBuilder::filename_pattern)
    /// - [`imu`](CaptureSpecBuilder::imu)
    /// - [`manifest_schema_version`](CaptureSpecBuilder::manifest_schema_version)
    /// - [`min_clip_duration_seconds`](CaptureSpecBuilder::min_clip_duration_seconds)
    /// - [`min_total_verified_duration_seconds`](CaptureSpecBuilder::min_total_verified_duration_seconds)
    /// - [`required_metadata_fields`](CaptureSpecBuilder::required_metadata_fields)
    /// - [`single_continuous_take`](CaptureSpecBuilder::single_continuous_take)
    /// - [`video`](CaptureSpecBuilder::video)
    pub fn build(self) -> Result<CaptureSpec, BuildError> {
        Ok(CaptureSpec {
            filename_pattern: self
                .filename_pattern
                .ok_or_else(|| BuildError::missing_field("filename_pattern"))?,
            imu: self.imu.ok_or_else(|| BuildError::missing_field("imu"))?,
            manifest_schema_version: self
                .manifest_schema_version
                .ok_or_else(|| BuildError::missing_field("manifest_schema_version"))?,
            min_clip_duration_seconds: self
                .min_clip_duration_seconds
                .ok_or_else(|| BuildError::missing_field("min_clip_duration_seconds"))?,
            min_total_verified_duration_seconds: self
                .min_total_verified_duration_seconds
                .ok_or_else(|| BuildError::missing_field("min_total_verified_duration_seconds"))?,
            required_metadata_fields: self
                .required_metadata_fields
                .ok_or_else(|| BuildError::missing_field("required_metadata_fields"))?,
            single_continuous_take: self
                .single_continuous_take
                .ok_or_else(|| BuildError::missing_field("single_continuous_take"))?,
            video: self
                .video
                .ok_or_else(|| BuildError::missing_field("video"))?,
        })
    }
}
