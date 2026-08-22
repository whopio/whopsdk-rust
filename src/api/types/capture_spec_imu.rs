pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CaptureSpecImu {
    /// Units for the device-motion channels, as a compact key=unit string.
    #[serde(default)]
    pub device_motion_units: String,
    /// Units for the magnetometer channel.
    #[serde(default)]
    pub magnetometer_units: String,
    /// Minimum sustained IMU sample rate in hertz for a clip to pass validation.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub min_rate_hz: f64,
    /// Target IMU sample rate in hertz.
    #[serde(default)]
    pub target_rate_hz: i64,
    /// Minimum IMU sample rate in hertz tolerated during the warmup window.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub warmup_min_rate_hz: f64,
    /// Startup window, in nanoseconds, during which the relaxed warmup rate applies.
    #[serde(default)]
    pub warmup_ns: i64,
}

impl CaptureSpecImu {
    pub fn builder() -> CaptureSpecImuBuilder {
        <CaptureSpecImuBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptureSpecImuBuilder {
    device_motion_units: Option<String>,
    magnetometer_units: Option<String>,
    min_rate_hz: Option<f64>,
    target_rate_hz: Option<i64>,
    warmup_min_rate_hz: Option<f64>,
    warmup_ns: Option<i64>,
}

impl CaptureSpecImuBuilder {
    pub fn device_motion_units(mut self, value: impl Into<String>) -> Self {
        self.device_motion_units = Some(value.into());
        self
    }

    pub fn magnetometer_units(mut self, value: impl Into<String>) -> Self {
        self.magnetometer_units = Some(value.into());
        self
    }

    pub fn min_rate_hz(mut self, value: f64) -> Self {
        self.min_rate_hz = Some(value);
        self
    }

    pub fn target_rate_hz(mut self, value: i64) -> Self {
        self.target_rate_hz = Some(value);
        self
    }

    pub fn warmup_min_rate_hz(mut self, value: f64) -> Self {
        self.warmup_min_rate_hz = Some(value);
        self
    }

    pub fn warmup_ns(mut self, value: i64) -> Self {
        self.warmup_ns = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptureSpecImu`].
    /// This method will fail if any of the following fields are not set:
    /// - [`device_motion_units`](CaptureSpecImuBuilder::device_motion_units)
    /// - [`magnetometer_units`](CaptureSpecImuBuilder::magnetometer_units)
    /// - [`min_rate_hz`](CaptureSpecImuBuilder::min_rate_hz)
    /// - [`target_rate_hz`](CaptureSpecImuBuilder::target_rate_hz)
    /// - [`warmup_min_rate_hz`](CaptureSpecImuBuilder::warmup_min_rate_hz)
    /// - [`warmup_ns`](CaptureSpecImuBuilder::warmup_ns)
    pub fn build(self) -> Result<CaptureSpecImu, BuildError> {
        Ok(CaptureSpecImu {
            device_motion_units: self
                .device_motion_units
                .ok_or_else(|| BuildError::missing_field("device_motion_units"))?,
            magnetometer_units: self
                .magnetometer_units
                .ok_or_else(|| BuildError::missing_field("magnetometer_units"))?,
            min_rate_hz: self
                .min_rate_hz
                .ok_or_else(|| BuildError::missing_field("min_rate_hz"))?,
            target_rate_hz: self
                .target_rate_hz
                .ok_or_else(|| BuildError::missing_field("target_rate_hz"))?,
            warmup_min_rate_hz: self
                .warmup_min_rate_hz
                .ok_or_else(|| BuildError::missing_field("warmup_min_rate_hz"))?,
            warmup_ns: self
                .warmup_ns
                .ok_or_else(|| BuildError::missing_field("warmup_ns"))?,
        })
    }
}
