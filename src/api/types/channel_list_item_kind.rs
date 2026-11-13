pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChannelListItemKind {
    Private,
    Community,
    Support,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ChannelListItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Private => serializer.serialize_str("private"),
            Self::Community => serializer.serialize_str("community"),
            Self::Support => serializer.serialize_str("support"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ChannelListItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "private" => Ok(Self::Private),
            "community" => Ok(Self::Community),
            "support" => Ok(Self::Support),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ChannelListItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Private => write!(f, "private"),
            Self::Community => write!(f, "community"),
            Self::Support => write!(f, "support"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
