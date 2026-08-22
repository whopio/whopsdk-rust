pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LeaderboardPartnersRequestPeriod {
    Day,
    Month,
    Year,
    Last30Days,
    AllTime,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LeaderboardPartnersRequestPeriod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Day => serializer.serialize_str("day"),
            Self::Month => serializer.serialize_str("month"),
            Self::Year => serializer.serialize_str("year"),
            Self::Last30Days => serializer.serialize_str("last_30_days"),
            Self::AllTime => serializer.serialize_str("all_time"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LeaderboardPartnersRequestPeriod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "day" => Ok(Self::Day),
            "month" => Ok(Self::Month),
            "year" => Ok(Self::Year),
            "last_30_days" => Ok(Self::Last30Days),
            "all_time" => Ok(Self::AllTime),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LeaderboardPartnersRequestPeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Day => write!(f, "day"),
            Self::Month => write!(f, "month"),
            Self::Year => write!(f, "year"),
            Self::Last30Days => write!(f, "last_30_days"),
            Self::AllTime => write!(f, "all_time"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
