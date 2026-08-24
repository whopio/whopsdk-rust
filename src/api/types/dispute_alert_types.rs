pub use crate::prelude::*;

/// The type of dispute alert.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DisputeAlertTypes {
    Dispute,
    DisputeRdr,
    Fraud,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DisputeAlertTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Dispute => serializer.serialize_str("dispute"),
            Self::DisputeRdr => serializer.serialize_str("dispute_rdr"),
            Self::Fraud => serializer.serialize_str("fraud"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DisputeAlertTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "dispute" => Ok(Self::Dispute),
            "dispute_rdr" => Ok(Self::DisputeRdr),
            "fraud" => Ok(Self::Fraud),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DisputeAlertTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dispute => write!(f, "dispute"),
            Self::DisputeRdr => write!(f, "dispute_rdr"),
            Self::Fraud => write!(f, "fraud"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
