pub use crate::prelude::*;

/// Why evidence can no longer be edited. `null` while `evidence_editable` is true.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DisputeEvidenceLockedReason {
    Submitted,
    ResponseWindowClosed,
    NotContestable,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DisputeEvidenceLockedReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Submitted => serializer.serialize_str("submitted"),
            Self::ResponseWindowClosed => serializer.serialize_str("response_window_closed"),
            Self::NotContestable => serializer.serialize_str("not_contestable"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DisputeEvidenceLockedReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "submitted" => Ok(Self::Submitted),
            "response_window_closed" => Ok(Self::ResponseWindowClosed),
            "not_contestable" => Ok(Self::NotContestable),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DisputeEvidenceLockedReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Submitted => write!(f, "submitted"),
            Self::ResponseWindowClosed => write!(f, "response_window_closed"),
            Self::NotContestable => write!(f, "not_contestable"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
