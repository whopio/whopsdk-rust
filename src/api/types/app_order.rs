pub use crate::prelude::*;

/// The order to fetch the apps in for discovery.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppOrder {
    CreatedAt,
    DiscoverableAt,
    TotalInstallsLast30Days,
    TotalInstallsLast7Days,
    TimeSpent,
    TimeSpentLast24Hours,
    DailyActiveUsers,
    AiPromptCount,
    TotalAiCostUsd,
    TotalAiTokens,
    LastAiPromptAt,
    AiAverageRating,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AppOrder {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::DiscoverableAt => serializer.serialize_str("discoverable_at"),
            Self::TotalInstallsLast30Days => {
                serializer.serialize_str("total_installs_last_30_days")
            }
            Self::TotalInstallsLast7Days => serializer.serialize_str("total_installs_last_7_days"),
            Self::TimeSpent => serializer.serialize_str("time_spent"),
            Self::TimeSpentLast24Hours => serializer.serialize_str("time_spent_last_24_hours"),
            Self::DailyActiveUsers => serializer.serialize_str("daily_active_users"),
            Self::AiPromptCount => serializer.serialize_str("ai_prompt_count"),
            Self::TotalAiCostUsd => serializer.serialize_str("total_ai_cost_usd"),
            Self::TotalAiTokens => serializer.serialize_str("total_ai_tokens"),
            Self::LastAiPromptAt => serializer.serialize_str("last_ai_prompt_at"),
            Self::AiAverageRating => serializer.serialize_str("ai_average_rating"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AppOrder {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created_at" => Ok(Self::CreatedAt),
            "discoverable_at" => Ok(Self::DiscoverableAt),
            "total_installs_last_30_days" => Ok(Self::TotalInstallsLast30Days),
            "total_installs_last_7_days" => Ok(Self::TotalInstallsLast7Days),
            "time_spent" => Ok(Self::TimeSpent),
            "time_spent_last_24_hours" => Ok(Self::TimeSpentLast24Hours),
            "daily_active_users" => Ok(Self::DailyActiveUsers),
            "ai_prompt_count" => Ok(Self::AiPromptCount),
            "total_ai_cost_usd" => Ok(Self::TotalAiCostUsd),
            "total_ai_tokens" => Ok(Self::TotalAiTokens),
            "last_ai_prompt_at" => Ok(Self::LastAiPromptAt),
            "ai_average_rating" => Ok(Self::AiAverageRating),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AppOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::DiscoverableAt => write!(f, "discoverable_at"),
            Self::TotalInstallsLast30Days => write!(f, "total_installs_last_30_days"),
            Self::TotalInstallsLast7Days => write!(f, "total_installs_last_7_days"),
            Self::TimeSpent => write!(f, "time_spent"),
            Self::TimeSpentLast24Hours => write!(f, "time_spent_last_24_hours"),
            Self::DailyActiveUsers => write!(f, "daily_active_users"),
            Self::AiPromptCount => write!(f, "ai_prompt_count"),
            Self::TotalAiCostUsd => write!(f, "total_ai_cost_usd"),
            Self::TotalAiTokens => write!(f, "total_ai_tokens"),
            Self::LastAiPromptAt => write!(f, "last_ai_prompt_at"),
            Self::AiAverageRating => write!(f, "ai_average_rating"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
