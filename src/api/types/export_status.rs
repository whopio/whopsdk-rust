pub use crate::prelude::*;

/// `pending` or `processing` while the file is generated, `completed` when the download is ready, `failed` if it errored, `expired` once the file has been deleted.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExportStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Expired,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ExportStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Processing => serializer.serialize_str("processing"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ExportStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "processing" => Ok(Self::Processing),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "expired" => Ok(Self::Expired),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ExportStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Processing => write!(f, "processing"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Expired => write!(f, "expired"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
