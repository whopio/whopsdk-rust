pub use crate::prelude::*;

/// Controls whether the app is published on Whop discovery or accessible only through its direct link. Publishing requires a name, icon, and description.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateAppsRequestStatus {
    Live,
    Unlisted,
    Hidden,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateAppsRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Live => serializer.serialize_str("live"),
            Self::Unlisted => serializer.serialize_str("unlisted"),
            Self::Hidden => serializer.serialize_str("hidden"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateAppsRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "live" => Ok(Self::Live),
            "unlisted" => Ok(Self::Unlisted),
            "hidden" => Ok(Self::Hidden),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateAppsRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Live => write!(f, "live"),
            Self::Unlisted => write!(f, "unlisted"),
            Self::Hidden => write!(f, "hidden"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
