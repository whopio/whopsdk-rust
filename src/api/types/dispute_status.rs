pub use crate::prelude::*;

/// Where the dispute stands. `needs_response` is awaiting evidence, `under_review` is with the processor, `won` returned the funds to the seller, `lost` returned them to the customer, and `closed` ended without a ruling. A dispute past its `evidence_due_at` reports `under_review` — the window to respond has closed.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DisputeStatus {
    NeedsResponse,
    UnderReview,
    Won,
    Lost,
    Closed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DisputeStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NeedsResponse => serializer.serialize_str("needs_response"),
            Self::UnderReview => serializer.serialize_str("under_review"),
            Self::Won => serializer.serialize_str("won"),
            Self::Lost => serializer.serialize_str("lost"),
            Self::Closed => serializer.serialize_str("closed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DisputeStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "needs_response" => Ok(Self::NeedsResponse),
            "under_review" => Ok(Self::UnderReview),
            "won" => Ok(Self::Won),
            "lost" => Ok(Self::Lost),
            "closed" => Ok(Self::Closed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DisputeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NeedsResponse => write!(f, "needs_response"),
            Self::UnderReview => write!(f, "under_review"),
            Self::Won => write!(f, "won"),
            Self::Lost => write!(f, "lost"),
            Self::Closed => write!(f, "closed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
