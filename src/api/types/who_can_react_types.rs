pub use crate::prelude::*;

/// Who can react on a chat feed
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WhoCanReactTypes {
    Everyone,
    NoOne,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WhoCanReactTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Everyone => serializer.serialize_str("everyone"),
            Self::NoOne => serializer.serialize_str("no_one"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WhoCanReactTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "everyone" => Ok(Self::Everyone),
            "no_one" => Ok(Self::NoOne),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WhoCanReactTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Everyone => write!(f, "everyone"),
            Self::NoOne => write!(f, "no_one"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
