pub use crate::prelude::*;

/// The reason funds are held: `reserve`, `bnpl`, `sequra`, `fraud_hold`, or `preshipment_hold`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentHoldType {
    Reserve,
    Bnpl,
    Sequra,
    FraudHold,
    PreshipmentHold,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentHoldType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Reserve => serializer.serialize_str("reserve"),
            Self::Bnpl => serializer.serialize_str("bnpl"),
            Self::Sequra => serializer.serialize_str("sequra"),
            Self::FraudHold => serializer.serialize_str("fraud_hold"),
            Self::PreshipmentHold => serializer.serialize_str("preshipment_hold"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentHoldType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "reserve" => Ok(Self::Reserve),
            "bnpl" => Ok(Self::Bnpl),
            "sequra" => Ok(Self::Sequra),
            "fraud_hold" => Ok(Self::FraudHold),
            "preshipment_hold" => Ok(Self::PreshipmentHold),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentHoldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reserve => write!(f, "reserve"),
            Self::Bnpl => write!(f, "bnpl"),
            Self::Sequra => write!(f, "sequra"),
            Self::FraudHold => write!(f, "fraud_hold"),
            Self::PreshipmentHold => write!(f, "preshipment_hold"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
