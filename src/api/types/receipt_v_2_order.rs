pub use crate::prelude::*;

/// The order to sort the results by.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReceiptV2Order {
    FinalAmount,
    CreatedAt,
    PaidAt,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReceiptV2Order {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::FinalAmount => serializer.serialize_str("final_amount"),
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::PaidAt => serializer.serialize_str("paid_at"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReceiptV2Order {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "final_amount" => Ok(Self::FinalAmount),
            "created_at" => Ok(Self::CreatedAt),
            "paid_at" => Ok(Self::PaidAt),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReceiptV2Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FinalAmount => write!(f, "final_amount"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::PaidAt => write!(f, "paid_at"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
