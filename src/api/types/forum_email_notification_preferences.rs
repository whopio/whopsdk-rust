pub use crate::prelude::*;

/// Email notification preference option for a forum feed
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForumEmailNotificationPreferences {
    AllAdminPosts,
    OnlyWeeklySummary,
    None,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ForumEmailNotificationPreferences {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AllAdminPosts => serializer.serialize_str("all_admin_posts"),
            Self::OnlyWeeklySummary => serializer.serialize_str("only_weekly_summary"),
            Self::None => serializer.serialize_str("none"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ForumEmailNotificationPreferences {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "all_admin_posts" => Ok(Self::AllAdminPosts),
            "only_weekly_summary" => Ok(Self::OnlyWeeklySummary),
            "none" => Ok(Self::None),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ForumEmailNotificationPreferences {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AllAdminPosts => write!(f, "all_admin_posts"),
            Self::OnlyWeeklySummary => write!(f, "only_weekly_summary"),
            Self::None => write!(f, "none"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
