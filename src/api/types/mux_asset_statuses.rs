pub use crate::prelude::*;

/// Mux asset statuses
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MuxAssetStatuses {
    Uploading,
    Created,
    Ready,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MuxAssetStatuses {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Uploading => serializer.serialize_str("uploading"),
            Self::Created => serializer.serialize_str("created"),
            Self::Ready => serializer.serialize_str("ready"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MuxAssetStatuses {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "uploading" => Ok(Self::Uploading),
            "created" => Ok(Self::Created),
            "ready" => Ok(Self::Ready),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MuxAssetStatuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uploading => write!(f, "uploading"),
            Self::Created => write!(f, "created"),
            Self::Ready => write!(f, "ready"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
