pub use crate::prelude::*;

/// `pending_upload` until the document has been relayed for review; `submitted` afterwards.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrieveVerificationsResponseRequiredDocumentsItemStatus {
    PendingUpload,
    Submitted,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrieveVerificationsResponseRequiredDocumentsItemStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PendingUpload => serializer.serialize_str("pending_upload"),
            Self::Submitted => serializer.serialize_str("submitted"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RetrieveVerificationsResponseRequiredDocumentsItemStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending_upload" => Ok(Self::PendingUpload),
            "submitted" => Ok(Self::Submitted),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrieveVerificationsResponseRequiredDocumentsItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PendingUpload => write!(f, "pending_upload"),
            Self::Submitted => write!(f, "submitted"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
