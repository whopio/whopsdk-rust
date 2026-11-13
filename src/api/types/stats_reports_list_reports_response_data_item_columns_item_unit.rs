pub use crate::prelude::*;

/// Metrics only. `money` is a Money object, `rate` a decimal fraction, the rest numbers.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListReportsResponseDataItemColumnsItemUnit {
    Money,
    Count,
    Rate,
    Months,
    Years,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListReportsResponseDataItemColumnsItemUnit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Money => serializer.serialize_str("money"),
            Self::Count => serializer.serialize_str("count"),
            Self::Rate => serializer.serialize_str("rate"),
            Self::Months => serializer.serialize_str("months"),
            Self::Years => serializer.serialize_str("years"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListReportsResponseDataItemColumnsItemUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "money" => Ok(Self::Money),
            "count" => Ok(Self::Count),
            "rate" => Ok(Self::Rate),
            "months" => Ok(Self::Months),
            "years" => Ok(Self::Years),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListReportsResponseDataItemColumnsItemUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Money => write!(f, "money"),
            Self::Count => write!(f, "count"),
            Self::Rate => write!(f, "rate"),
            Self::Months => write!(f, "months"),
            Self::Years => write!(f, "years"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
