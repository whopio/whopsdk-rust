pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrieveBreakdownResponseItemsItemAvatarShape {
    Circle,
    Square,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrieveBreakdownResponseItemsItemAvatarShape {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Circle => serializer.serialize_str("circle"),
            Self::Square => serializer.serialize_str("square"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RetrieveBreakdownResponseItemsItemAvatarShape {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "circle" => Ok(Self::Circle),
            "square" => Ok(Self::Square),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrieveBreakdownResponseItemsItemAvatarShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Circle => write!(f, "circle"),
            Self::Square => write!(f, "square"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
