pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListFinancialActivityRequestBalanceType {
    Total,
    Available,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListFinancialActivityRequestBalanceType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Total => serializer.serialize_str("total"),
            Self::Available => serializer.serialize_str("available"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListFinancialActivityRequestBalanceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "total" => Ok(Self::Total),
            "available" => Ok(Self::Available),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListFinancialActivityRequestBalanceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Total => write!(f, "total"),
            Self::Available => write!(f, "available"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
