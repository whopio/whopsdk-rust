pub use crate::prelude::*;

/// Properties only. `fixed` breakdowns list every value, zero-filled; `open` ones list values with activity.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListReportsResponseDataItemColumnsItemSet {
    Fixed,
    Open,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListReportsResponseDataItemColumnsItemSet {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Fixed => serializer.serialize_str("fixed"),
            Self::Open => serializer.serialize_str("open"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListReportsResponseDataItemColumnsItemSet {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "fixed" => Ok(Self::Fixed),
            "open" => Ok(Self::Open),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListReportsResponseDataItemColumnsItemSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fixed => write!(f, "fixed"),
            Self::Open => write!(f, "open"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
