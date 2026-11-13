pub use crate::prelude::*;

/// `range` reports measure activity in a period; `as_of` reports measure state at a moment.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListReportsResponseDataItemWindowKind {
    Range,
    AsOf,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListReportsResponseDataItemWindowKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Range => serializer.serialize_str("range"),
            Self::AsOf => serializer.serialize_str("as_of"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListReportsResponseDataItemWindowKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "range" => Ok(Self::Range),
            "as_of" => Ok(Self::AsOf),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListReportsResponseDataItemWindowKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Range => write!(f, "range"),
            Self::AsOf => write!(f, "as_of"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
