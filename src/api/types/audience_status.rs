pub use crate::prelude::*;

/// Current state of the audience import. `syncing` means Whop is sending matched rows to connected ad accounts. When status is `partial` or `failed`, `error_message` explains what went wrong.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AudienceStatus {
    Pending,
    Processing,
    Syncing,
    Ready,
    Partial,
    Failed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AudienceStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Processing => serializer.serialize_str("processing"),
            Self::Syncing => serializer.serialize_str("syncing"),
            Self::Ready => serializer.serialize_str("ready"),
            Self::Partial => serializer.serialize_str("partial"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AudienceStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "processing" => Ok(Self::Processing),
            "syncing" => Ok(Self::Syncing),
            "ready" => Ok(Self::Ready),
            "partial" => Ok(Self::Partial),
            "failed" => Ok(Self::Failed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AudienceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Processing => write!(f, "processing"),
            Self::Syncing => write!(f, "syncing"),
            Self::Ready => write!(f, "ready"),
            Self::Partial => write!(f, "partial"),
            Self::Failed => write!(f, "failed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
