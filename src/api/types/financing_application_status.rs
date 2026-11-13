pub use crate::prelude::*;

/// The application's review state. Approval does not by itself indicate that financing payment methods are enabled on the account.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FinancingApplicationStatus {
    RequiresCollection,
    AwaitingReview,
    Completed,
    Approved,
    Denied,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FinancingApplicationStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::RequiresCollection => serializer.serialize_str("requires_collection"),
            Self::AwaitingReview => serializer.serialize_str("awaiting_review"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::Approved => serializer.serialize_str("approved"),
            Self::Denied => serializer.serialize_str("denied"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FinancingApplicationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "requires_collection" => Ok(Self::RequiresCollection),
            "awaiting_review" => Ok(Self::AwaitingReview),
            "completed" => Ok(Self::Completed),
            "approved" => Ok(Self::Approved),
            "denied" => Ok(Self::Denied),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FinancingApplicationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequiresCollection => write!(f, "requires_collection"),
            Self::AwaitingReview => write!(f, "awaiting_review"),
            Self::Completed => write!(f, "completed"),
            Self::Approved => write!(f, "approved"),
            Self::Denied => write!(f, "denied"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
