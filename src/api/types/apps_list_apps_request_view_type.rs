pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListAppsRequestViewType {
    Hub,
    Discover,
    Dash,
    Dashboard,
    Analytics,
    Skills,
    Openapi,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListAppsRequestViewType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Hub => serializer.serialize_str("hub"),
            Self::Discover => serializer.serialize_str("discover"),
            Self::Dash => serializer.serialize_str("dash"),
            Self::Dashboard => serializer.serialize_str("dashboard"),
            Self::Analytics => serializer.serialize_str("analytics"),
            Self::Skills => serializer.serialize_str("skills"),
            Self::Openapi => serializer.serialize_str("openapi"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListAppsRequestViewType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "hub" => Ok(Self::Hub),
            "discover" => Ok(Self::Discover),
            "dash" => Ok(Self::Dash),
            "dashboard" => Ok(Self::Dashboard),
            "analytics" => Ok(Self::Analytics),
            "skills" => Ok(Self::Skills),
            "openapi" => Ok(Self::Openapi),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListAppsRequestViewType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hub => write!(f, "hub"),
            Self::Discover => write!(f, "discover"),
            Self::Dash => write!(f, "dash"),
            Self::Dashboard => write!(f, "dashboard"),
            Self::Analytics => write!(f, "analytics"),
            Self::Skills => write!(f, "skills"),
            Self::Openapi => write!(f, "openapi"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
