pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListVerificationsResponseDataItemKind {
    Individual,
    Business,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListVerificationsResponseDataItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Individual => serializer.serialize_str("individual"),
            Self::Business => serializer.serialize_str("business"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListVerificationsResponseDataItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "individual" => Ok(Self::Individual),
            "business" => Ok(Self::Business),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListVerificationsResponseDataItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Individual => write!(f, "individual"),
            Self::Business => write!(f, "business"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
