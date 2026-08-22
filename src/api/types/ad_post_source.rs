pub use crate::prelude::*;

/// Identifies the network that owns `post_id`; `null` when the ad uses uploaded creatives.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdPostSource {
    Facebook,
    Instagram,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdPostSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Facebook => serializer.serialize_str("facebook"),
            Self::Instagram => serializer.serialize_str("instagram"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdPostSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "facebook" => Ok(Self::Facebook),
            "instagram" => Ok(Self::Instagram),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdPostSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Facebook => write!(f, "facebook"),
            Self::Instagram => write!(f, "instagram"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
