pub use crate::prelude::*;

/// Your inbox state for a private channel. `null` for other users and other channel kinds.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChannelMemberInboxStatus {
    Requested,
    Accepted,
    Hidden,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ChannelMemberInboxStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Requested => serializer.serialize_str("requested"),
            Self::Accepted => serializer.serialize_str("accepted"),
            Self::Hidden => serializer.serialize_str("hidden"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ChannelMemberInboxStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "requested" => Ok(Self::Requested),
            "accepted" => Ok(Self::Accepted),
            "hidden" => Ok(Self::Hidden),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ChannelMemberInboxStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Requested => write!(f, "requested"),
            Self::Accepted => write!(f, "accepted"),
            Self::Hidden => write!(f, "hidden"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
