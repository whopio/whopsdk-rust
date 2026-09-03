pub use crate::prelude::*;

/// Where the refund stands with the processor: `pending`, `requires_action`, `succeeded`, `failed`, or `canceled`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RefundStatus {
    Pending,
    RequiresAction,
    Succeeded,
    Failed,
    Canceled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RefundStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::RequiresAction => serializer.serialize_str("requires_action"),
            Self::Succeeded => serializer.serialize_str("succeeded"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RefundStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "requires_action" => Ok(Self::RequiresAction),
            "succeeded" => Ok(Self::Succeeded),
            "failed" => Ok(Self::Failed),
            "canceled" => Ok(Self::Canceled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RefundStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::RequiresAction => write!(f, "requires_action"),
            Self::Succeeded => write!(f, "succeeded"),
            Self::Failed => write!(f, "failed"),
            Self::Canceled => write!(f, "canceled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
