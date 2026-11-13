pub use crate::prelude::*;

/// `needs_response` awaits evidence (after a processor refused a filing, `submission_failure_reason` says why); `generating_response` has evidence with the processor, not yet accepted; `under_review` is with the processor; `won`/`lost` are the rulings; `prevented` settles like `lost`. `warning_` statuses are the inquiry tiers. Past `evidence_due_at` an open one reports `under_review`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DisputeStatus {
    NeedsResponse,
    WarningNeedsResponse,
    UnderReview,
    WarningUnderReview,
    GeneratingResponse,
    Won,
    Lost,
    Prevented,
    WarningClosed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DisputeStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NeedsResponse => serializer.serialize_str("needs_response"),
            Self::WarningNeedsResponse => serializer.serialize_str("warning_needs_response"),
            Self::UnderReview => serializer.serialize_str("under_review"),
            Self::WarningUnderReview => serializer.serialize_str("warning_under_review"),
            Self::GeneratingResponse => serializer.serialize_str("generating_response"),
            Self::Won => serializer.serialize_str("won"),
            Self::Lost => serializer.serialize_str("lost"),
            Self::Prevented => serializer.serialize_str("prevented"),
            Self::WarningClosed => serializer.serialize_str("warning_closed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DisputeStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "needs_response" => Ok(Self::NeedsResponse),
            "warning_needs_response" => Ok(Self::WarningNeedsResponse),
            "under_review" => Ok(Self::UnderReview),
            "warning_under_review" => Ok(Self::WarningUnderReview),
            "generating_response" => Ok(Self::GeneratingResponse),
            "won" => Ok(Self::Won),
            "lost" => Ok(Self::Lost),
            "prevented" => Ok(Self::Prevented),
            "warning_closed" => Ok(Self::WarningClosed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DisputeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NeedsResponse => write!(f, "needs_response"),
            Self::WarningNeedsResponse => write!(f, "warning_needs_response"),
            Self::UnderReview => write!(f, "under_review"),
            Self::WarningUnderReview => write!(f, "warning_under_review"),
            Self::GeneratingResponse => write!(f, "generating_response"),
            Self::Won => write!(f, "won"),
            Self::Lost => write!(f, "lost"),
            Self::Prevented => write!(f, "prevented"),
            Self::WarningClosed => write!(f, "warning_closed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
