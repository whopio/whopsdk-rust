pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AcceptCommissionAdvanceBusinessesRequestAcknowledgmentsItem {
    CommissionReplacement,
    Clawback,
    Term,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AcceptCommissionAdvanceBusinessesRequestAcknowledgmentsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CommissionReplacement => serializer.serialize_str("commission_replacement"),
            Self::Clawback => serializer.serialize_str("clawback"),
            Self::Term => serializer.serialize_str("term"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AcceptCommissionAdvanceBusinessesRequestAcknowledgmentsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "commission_replacement" => Ok(Self::CommissionReplacement),
            "clawback" => Ok(Self::Clawback),
            "term" => Ok(Self::Term),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AcceptCommissionAdvanceBusinessesRequestAcknowledgmentsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommissionReplacement => write!(f, "commission_replacement"),
            Self::Clawback => write!(f, "clawback"),
            Self::Term => write!(f, "term"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
