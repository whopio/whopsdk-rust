pub use crate::prelude::*;

/// The lifecycle status of a card transaction.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CardIssuingTransactionStatus {
    Pending,
    Completed,
    Reversed,
    Declined,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CardIssuingTransactionStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::Reversed => serializer.serialize_str("reversed"),
            Self::Declined => serializer.serialize_str("declined"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CardIssuingTransactionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "completed" => Ok(Self::Completed),
            "reversed" => Ok(Self::Reversed),
            "declined" => Ok(Self::Declined),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CardIssuingTransactionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Completed => write!(f, "completed"),
            Self::Reversed => write!(f, "reversed"),
            Self::Declined => write!(f, "declined"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
