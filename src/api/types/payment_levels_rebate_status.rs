pub use crate::prelude::*;

/// `rebated` when the payment earned part of its account's monthly Whop Levels rebate. `pending` until the month its processing fee posted in has closed. `not_covered` when the account's spend that month ran out before this payment. `ineligible` when it can't earn one; `ineligible_reason` says why.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentLevelsRebateStatus {
    Rebated,
    Pending,
    NotCovered,
    Ineligible,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentLevelsRebateStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Rebated => serializer.serialize_str("rebated"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::NotCovered => serializer.serialize_str("not_covered"),
            Self::Ineligible => serializer.serialize_str("ineligible"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentLevelsRebateStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "rebated" => Ok(Self::Rebated),
            "pending" => Ok(Self::Pending),
            "not_covered" => Ok(Self::NotCovered),
            "ineligible" => Ok(Self::Ineligible),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentLevelsRebateStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rebated => write!(f, "rebated"),
            Self::Pending => write!(f, "pending"),
            Self::NotCovered => write!(f, "not_covered"),
            Self::Ineligible => write!(f, "ineligible"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
