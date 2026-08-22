pub use crate::prelude::*;

/// Why this part of the balance is held. `regular` is the account's standing risk reserve; `bnpl` and `sequra` cover buy-now-pay-later settlement; `preshipment_hold` covers a physical order that has not shipped yet; `fraud_hold` is held while activity is reviewed.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountReserveTypeType {
    Regular,
    Bnpl,
    Sequra,
    FraudHold,
    PreshipmentHold,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountReserveTypeType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Regular => serializer.serialize_str("regular"),
            Self::Bnpl => serializer.serialize_str("bnpl"),
            Self::Sequra => serializer.serialize_str("sequra"),
            Self::FraudHold => serializer.serialize_str("fraud_hold"),
            Self::PreshipmentHold => serializer.serialize_str("preshipment_hold"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountReserveTypeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "regular" => Ok(Self::Regular),
            "bnpl" => Ok(Self::Bnpl),
            "sequra" => Ok(Self::Sequra),
            "fraud_hold" => Ok(Self::FraudHold),
            "preshipment_hold" => Ok(Self::PreshipmentHold),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountReserveTypeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Regular => write!(f, "regular"),
            Self::Bnpl => write!(f, "bnpl"),
            Self::Sequra => write!(f, "sequra"),
            Self::FraudHold => write!(f, "fraud_hold"),
            Self::PreshipmentHold => write!(f, "preshipment_hold"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
