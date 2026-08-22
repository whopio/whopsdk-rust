pub use crate::prelude::*;

/// The type of object. Always `experience_notification_preference`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ExperienceNotificationPreferenceObject {
    #[serde(rename = "experience_notification_preference")]
    ExperienceNotificationPreference,
}
impl fmt::Display for ExperienceNotificationPreferenceObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ExperienceNotificationPreference => "experience_notification_preference",
        };
        write!(f, "{}", s)
    }
}
