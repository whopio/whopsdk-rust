pub use crate::prelude::*;

/// The kind of media this asset holds.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MediaAssetMediaType {
    Video,
    Image,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MediaAssetMediaType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Video => serializer.serialize_str("video"),
            Self::Image => serializer.serialize_str("image"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MediaAssetMediaType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "video" => Ok(Self::Video),
            "image" => Ok(Self::Image),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MediaAssetMediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Video => write!(f, "video"),
            Self::Image => write!(f, "image"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
