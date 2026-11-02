pub use crate::prelude::*;

/// Unit of the earning window. Month means a calendar month; day means a day.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PartnerPayoutDurationUnit {
    Day,
    Month,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PartnerPayoutDurationUnit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Day => serializer.serialize_str("day"),
            Self::Month => serializer.serialize_str("month"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PartnerPayoutDurationUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "day" => Ok(Self::Day),
            "month" => Ok(Self::Month),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PartnerPayoutDurationUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Day => write!(f, "day"),
            Self::Month => write!(f, "month"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
