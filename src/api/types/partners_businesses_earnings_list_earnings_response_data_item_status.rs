pub use crate::prelude::*;

/// Current status of the earning.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListEarningsResponseDataItemStatus {
    AwaitingSettlement,
    Pending,
    Completed,
    Canceled,
    Reversed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListEarningsResponseDataItemStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AwaitingSettlement => serializer.serialize_str("awaiting_settlement"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::Reversed => serializer.serialize_str("reversed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListEarningsResponseDataItemStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "awaiting_settlement" => Ok(Self::AwaitingSettlement),
            "pending" => Ok(Self::Pending),
            "completed" => Ok(Self::Completed),
            "canceled" => Ok(Self::Canceled),
            "reversed" => Ok(Self::Reversed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListEarningsResponseDataItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AwaitingSettlement => write!(f, "awaiting_settlement"),
            Self::Pending => write!(f, "pending"),
            Self::Completed => write!(f, "completed"),
            Self::Canceled => write!(f, "canceled"),
            Self::Reversed => write!(f, "reversed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
