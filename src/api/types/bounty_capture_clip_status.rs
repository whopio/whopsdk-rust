pub use crate::prelude::*;

/// Recording and validation state. `recording` is still capturing; `verifying` is running server-side validation; `ready` passed validation and counts toward the verified-duration payout gate; `failed` did not validate.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BountyCaptureClipStatus {
    Recording,
    Verifying,
    Ready,
    Failed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BountyCaptureClipStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Recording => serializer.serialize_str("recording"),
            Self::Verifying => serializer.serialize_str("verifying"),
            Self::Ready => serializer.serialize_str("ready"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BountyCaptureClipStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "recording" => Ok(Self::Recording),
            "verifying" => Ok(Self::Verifying),
            "ready" => Ok(Self::Ready),
            "failed" => Ok(Self::Failed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BountyCaptureClipStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Recording => write!(f, "recording"),
            Self::Verifying => write!(f, "verifying"),
            Self::Ready => write!(f, "ready"),
            Self::Failed => write!(f, "failed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
