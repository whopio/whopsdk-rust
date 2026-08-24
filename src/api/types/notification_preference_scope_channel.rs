pub use crate::prelude::*;

/// Delivery channel the preference applies to. `null` applies to every channel.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NotificationPreferenceScopeChannel {
    InApp,
    Mobile,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for NotificationPreferenceScopeChannel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::InApp => serializer.serialize_str("in_app"),
            Self::Mobile => serializer.serialize_str("mobile"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for NotificationPreferenceScopeChannel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "in_app" => Ok(Self::InApp),
            "mobile" => Ok(Self::Mobile),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for NotificationPreferenceScopeChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InApp => write!(f, "in_app"),
            Self::Mobile => write!(f, "mobile"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
