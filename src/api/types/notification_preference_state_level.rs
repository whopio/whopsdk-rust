pub use crate::prelude::*;

/// What the user is now notified about in this scope, or `null` if the preference was cleared and the scope inherits its default again.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NotificationPreferenceStateLevel {
    All,
    Mentions,
    Nothing,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for NotificationPreferenceStateLevel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::All => serializer.serialize_str("all"),
            Self::Mentions => serializer.serialize_str("mentions"),
            Self::Nothing => serializer.serialize_str("nothing"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for NotificationPreferenceStateLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "all" => Ok(Self::All),
            "mentions" => Ok(Self::Mentions),
            "nothing" => Ok(Self::Nothing),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for NotificationPreferenceStateLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Mentions => write!(f, "mentions"),
            Self::Nothing => write!(f, "nothing"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
