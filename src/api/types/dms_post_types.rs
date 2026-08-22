pub use crate::prelude::*;

/// The types of post
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DmsPostTypes {
    Regular,
    System,
    Automated,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DmsPostTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Regular => serializer.serialize_str("regular"),
            Self::System => serializer.serialize_str("system"),
            Self::Automated => serializer.serialize_str("automated"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DmsPostTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "regular" => Ok(Self::Regular),
            "system" => Ok(Self::System),
            "automated" => Ok(Self::Automated),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DmsPostTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Regular => write!(f, "regular"),
            Self::System => write!(f, "system"),
            Self::Automated => write!(f, "automated"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
