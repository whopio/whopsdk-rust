pub use crate::prelude::*;

/// Whether the column is an attribute of the row or a number measured over it.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListReportsResponseDataItemColumnsItemType {
    Property,
    Metric,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListReportsResponseDataItemColumnsItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Property => serializer.serialize_str("property"),
            Self::Metric => serializer.serialize_str("metric"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListReportsResponseDataItemColumnsItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "property" => Ok(Self::Property),
            "metric" => Ok(Self::Metric),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListReportsResponseDataItemColumnsItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Property => write!(f, "property"),
            Self::Metric => write!(f, "metric"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
