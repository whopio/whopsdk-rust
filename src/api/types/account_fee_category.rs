pub use crate::prelude::*;

/// Which group of the fee schedule this fee belongs to, for grouping in a UI.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountFeeCategory {
    Payments,
    Disputes,
    Optimization,
    Payouts,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountFeeCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Payments => serializer.serialize_str("payments"),
            Self::Disputes => serializer.serialize_str("disputes"),
            Self::Optimization => serializer.serialize_str("optimization"),
            Self::Payouts => serializer.serialize_str("payouts"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountFeeCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "payments" => Ok(Self::Payments),
            "disputes" => Ok(Self::Disputes),
            "optimization" => Ok(Self::Optimization),
            "payouts" => Ok(Self::Payouts),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountFeeCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Payments => write!(f, "payments"),
            Self::Disputes => write!(f, "disputes"),
            Self::Optimization => write!(f, "optimization"),
            Self::Payouts => write!(f, "payouts"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
