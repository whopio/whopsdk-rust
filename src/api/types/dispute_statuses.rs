pub use crate::prelude::*;

/// The possible statuses of a dispute
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DisputeStatuses {
    WarningNeedsResponse,
    WarningUnderReview,
    WarningClosed,
    NeedsResponse,
    UnderReview,
    Won,
    Lost,
    Closed,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DisputeStatuses {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::WarningNeedsResponse => serializer.serialize_str("warning_needs_response"),
            Self::WarningUnderReview => serializer.serialize_str("warning_under_review"),
            Self::WarningClosed => serializer.serialize_str("warning_closed"),
            Self::NeedsResponse => serializer.serialize_str("needs_response"),
            Self::UnderReview => serializer.serialize_str("under_review"),
            Self::Won => serializer.serialize_str("won"),
            Self::Lost => serializer.serialize_str("lost"),
            Self::Closed => serializer.serialize_str("closed"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DisputeStatuses {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "warning_needs_response" => Ok(Self::WarningNeedsResponse),
            "warning_under_review" => Ok(Self::WarningUnderReview),
            "warning_closed" => Ok(Self::WarningClosed),
            "needs_response" => Ok(Self::NeedsResponse),
            "under_review" => Ok(Self::UnderReview),
            "won" => Ok(Self::Won),
            "lost" => Ok(Self::Lost),
            "closed" => Ok(Self::Closed),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DisputeStatuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WarningNeedsResponse => write!(f, "warning_needs_response"),
            Self::WarningUnderReview => write!(f, "warning_under_review"),
            Self::WarningClosed => write!(f, "warning_closed"),
            Self::NeedsResponse => write!(f, "needs_response"),
            Self::UnderReview => write!(f, "under_review"),
            Self::Won => write!(f, "won"),
            Self::Lost => write!(f, "lost"),
            Self::Closed => write!(f, "closed"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
