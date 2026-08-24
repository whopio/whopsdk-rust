pub use crate::prelude::*;

/// What a behavior category is measured on, on ad platforms that scope them. Send it back on the `detailed_targeting.behaviors` entry alongside the id. Null for options that aren't scoped.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DetailedTargetingOptionBehaviorType {
    Video,
    Creator,
    Hashtag,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DetailedTargetingOptionBehaviorType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Video => serializer.serialize_str("video"),
            Self::Creator => serializer.serialize_str("creator"),
            Self::Hashtag => serializer.serialize_str("hashtag"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DetailedTargetingOptionBehaviorType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "video" => Ok(Self::Video),
            "creator" => Ok(Self::Creator),
            "hashtag" => Ok(Self::Hashtag),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DetailedTargetingOptionBehaviorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Video => write!(f, "video"),
            Self::Creator => write!(f, "creator"),
            Self::Hashtag => write!(f, "hashtag"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
