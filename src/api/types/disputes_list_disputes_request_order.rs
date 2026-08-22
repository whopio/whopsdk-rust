pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListDisputesRequestOrder {
    CreatedAt,
    Amount,
    EvidenceDueAt,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListDisputesRequestOrder {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::Amount => serializer.serialize_str("amount"),
            Self::EvidenceDueAt => serializer.serialize_str("evidence_due_at"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListDisputesRequestOrder {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created_at" => Ok(Self::CreatedAt),
            "amount" => Ok(Self::Amount),
            "evidence_due_at" => Ok(Self::EvidenceDueAt),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListDisputesRequestOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::Amount => write!(f, "amount"),
            Self::EvidenceDueAt => write!(f, "evidence_due_at"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
