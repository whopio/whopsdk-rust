pub use crate::prelude::*;

/// Lifecycle state. `in_progress` submissions are active attempts that have not submitted proof yet; `submitted` submissions await review; `approved` submissions were accepted and paid; `denied` submissions were rejected. `null` when the attempt ended without proof, taking it out of the public lifecycle — those attempts are absent from every public list and read.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BountySubmissionStatus {
    InProgress,
    Submitted,
    Approved,
    Denied,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BountySubmissionStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::InProgress => serializer.serialize_str("in_progress"),
            Self::Submitted => serializer.serialize_str("submitted"),
            Self::Approved => serializer.serialize_str("approved"),
            Self::Denied => serializer.serialize_str("denied"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BountySubmissionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "in_progress" => Ok(Self::InProgress),
            "submitted" => Ok(Self::Submitted),
            "approved" => Ok(Self::Approved),
            "denied" => Ok(Self::Denied),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BountySubmissionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InProgress => write!(f, "in_progress"),
            Self::Submitted => write!(f, "submitted"),
            Self::Approved => write!(f, "approved"),
            Self::Denied => write!(f, "denied"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
