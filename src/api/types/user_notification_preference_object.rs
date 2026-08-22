pub use crate::prelude::*;

/// The type of object. Always `notification_preference`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UserNotificationPreferenceObject {
    #[serde(rename = "notification_preference")]
    NotificationPreference,
}
impl fmt::Display for UserNotificationPreferenceObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::NotificationPreference => "notification_preference",
        };
        write!(f, "{}", s)
    }
}
