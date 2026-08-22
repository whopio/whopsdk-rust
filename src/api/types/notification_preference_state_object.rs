pub use crate::prelude::*;

/// Which kind of preference was written: `experience_notification_preference` for an experience level, `notification_preference` for a topic override.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NotificationPreferenceStateObject {
    NotificationPreference,
    ExperienceNotificationPreference,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for NotificationPreferenceStateObject {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NotificationPreference => serializer.serialize_str("notification_preference"),
            Self::ExperienceNotificationPreference => {
                serializer.serialize_str("experience_notification_preference")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for NotificationPreferenceStateObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "notification_preference" => Ok(Self::NotificationPreference),
            "experience_notification_preference" => Ok(Self::ExperienceNotificationPreference),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for NotificationPreferenceStateObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotificationPreference => write!(f, "notification_preference"),
            Self::ExperienceNotificationPreference => {
                write!(f, "experience_notification_preference")
            }
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
