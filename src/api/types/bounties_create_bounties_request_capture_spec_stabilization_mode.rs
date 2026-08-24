pub use crate::prelude::*;

/// How the recorder configures video stabilization. `off` preserves raw motion for pose extraction.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateBountiesRequestCaptureSpecStabilizationMode {
    Off,
    On,
    Any,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateBountiesRequestCaptureSpecStabilizationMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Off => serializer.serialize_str("off"),
            Self::On => serializer.serialize_str("on"),
            Self::Any => serializer.serialize_str("any"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateBountiesRequestCaptureSpecStabilizationMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "off" => Ok(Self::Off),
            "on" => Ok(Self::On),
            "any" => Ok(Self::Any),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateBountiesRequestCaptureSpecStabilizationMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "off"),
            Self::On => write!(f, "on"),
            Self::Any => write!(f, "any"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
