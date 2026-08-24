pub use crate::prelude::*;

/// The statuses of a DMs feed member
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DmsFeedMemberStatuses {
    Requested,
    Accepted,
    Hidden,
    Closed,
    Archived,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DmsFeedMemberStatuses {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Requested => serializer.serialize_str("requested"),
            Self::Accepted => serializer.serialize_str("accepted"),
            Self::Hidden => serializer.serialize_str("hidden"),
            Self::Closed => serializer.serialize_str("closed"),
            Self::Archived => serializer.serialize_str("archived"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DmsFeedMemberStatuses {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "requested" => Ok(Self::Requested),
            "accepted" => Ok(Self::Accepted),
            "hidden" => Ok(Self::Hidden),
            "closed" => Ok(Self::Closed),
            "archived" => Ok(Self::Archived),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DmsFeedMemberStatuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Requested => write!(f, "requested"),
            Self::Accepted => write!(f, "accepted"),
            Self::Hidden => write!(f, "hidden"),
            Self::Closed => write!(f, "closed"),
            Self::Archived => write!(f, "archived"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
