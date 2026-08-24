pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListEarningsRequestOrder {
    CreatedAt,
    CommissionAmount,
    TransactionAmount,
    PayoutAt,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListEarningsRequestOrder {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::CommissionAmount => serializer.serialize_str("commission_amount"),
            Self::TransactionAmount => serializer.serialize_str("transaction_amount"),
            Self::PayoutAt => serializer.serialize_str("payout_at"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListEarningsRequestOrder {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created_at" => Ok(Self::CreatedAt),
            "commission_amount" => Ok(Self::CommissionAmount),
            "transaction_amount" => Ok(Self::TransactionAmount),
            "payout_at" => Ok(Self::PayoutAt),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListEarningsRequestOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::CommissionAmount => write!(f, "commission_amount"),
            Self::TransactionAmount => write!(f, "transaction_amount"),
            Self::PayoutAt => write!(f, "payout_at"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
