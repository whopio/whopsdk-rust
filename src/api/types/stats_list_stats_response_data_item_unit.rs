pub use crate::prelude::*;

/// How to read the metric's values: count is an integer, currency is a decimal amount, and percent is a number where 1.6 means 1.6%.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListStatsResponseDataItemUnit {
    Count,
    Currency,
    Percent,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListStatsResponseDataItemUnit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Count => serializer.serialize_str("count"),
            Self::Currency => serializer.serialize_str("currency"),
            Self::Percent => serializer.serialize_str("percent"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListStatsResponseDataItemUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "count" => Ok(Self::Count),
            "currency" => Ok(Self::Currency),
            "percent" => Ok(Self::Percent),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListStatsResponseDataItemUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Count => write!(f, "count"),
            Self::Currency => write!(f, "currency"),
            Self::Percent => write!(f, "percent"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
