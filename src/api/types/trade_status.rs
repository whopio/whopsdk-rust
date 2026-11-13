pub use crate::prelude::*;

/// `pending` until the trade starts. `processing` while it runs, also while an order that did not fill sends the money it bridged in back to the wallet. `completed` when it did what was asked. `failed` when it did not; `funds_location` says where the money is. `in_review` when the outcome is unknown and a person checks it.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TradeStatus {
    Pending,
    Processing,
    InReview,
    Completed,
    Failed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TradeStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Processing => serializer.serialize_str("processing"),
            Self::InReview => serializer.serialize_str("in_review"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TradeStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "processing" => Ok(Self::Processing),
            "in_review" => Ok(Self::InReview),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TradeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Processing => write!(f, "processing"),
            Self::InReview => write!(f, "in_review"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
