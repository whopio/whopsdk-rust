pub use crate::prelude::*;

/// Interaction that qualifies a person for this rule.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AudienceEngagementVideoRuleEvent {
    Watched3Seconds,
    Watched10Seconds,
    Watched15Seconds,
    Watched25Percent,
    Watched50Percent,
    Watched75Percent,
    Watched95Percent,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AudienceEngagementVideoRuleEvent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Watched3Seconds => serializer.serialize_str("watched_3_seconds"),
            Self::Watched10Seconds => serializer.serialize_str("watched_10_seconds"),
            Self::Watched15Seconds => serializer.serialize_str("watched_15_seconds"),
            Self::Watched25Percent => serializer.serialize_str("watched_25_percent"),
            Self::Watched50Percent => serializer.serialize_str("watched_50_percent"),
            Self::Watched75Percent => serializer.serialize_str("watched_75_percent"),
            Self::Watched95Percent => serializer.serialize_str("watched_95_percent"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AudienceEngagementVideoRuleEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "watched_3_seconds" => Ok(Self::Watched3Seconds),
            "watched_10_seconds" => Ok(Self::Watched10Seconds),
            "watched_15_seconds" => Ok(Self::Watched15Seconds),
            "watched_25_percent" => Ok(Self::Watched25Percent),
            "watched_50_percent" => Ok(Self::Watched50Percent),
            "watched_75_percent" => Ok(Self::Watched75Percent),
            "watched_95_percent" => Ok(Self::Watched95Percent),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AudienceEngagementVideoRuleEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Watched3Seconds => write!(f, "watched_3_seconds"),
            Self::Watched10Seconds => write!(f, "watched_10_seconds"),
            Self::Watched15Seconds => write!(f, "watched_15_seconds"),
            Self::Watched25Percent => write!(f, "watched_25_percent"),
            Self::Watched50Percent => write!(f, "watched_50_percent"),
            Self::Watched75Percent => write!(f, "watched_75_percent"),
            Self::Watched95Percent => write!(f, "watched_95_percent"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
