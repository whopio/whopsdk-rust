pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrieveStatsRequestInterval {
    Minute,
    FiveMinutes,
    ThirtyMinutes,
    Hour,
    Day,
    Week,
    Month,
    Year,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrieveStatsRequestInterval {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Minute => serializer.serialize_str("minute"),
            Self::FiveMinutes => serializer.serialize_str("five_minutes"),
            Self::ThirtyMinutes => serializer.serialize_str("thirty_minutes"),
            Self::Hour => serializer.serialize_str("hour"),
            Self::Day => serializer.serialize_str("day"),
            Self::Week => serializer.serialize_str("week"),
            Self::Month => serializer.serialize_str("month"),
            Self::Year => serializer.serialize_str("year"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RetrieveStatsRequestInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "minute" => Ok(Self::Minute),
            "five_minutes" => Ok(Self::FiveMinutes),
            "thirty_minutes" => Ok(Self::ThirtyMinutes),
            "hour" => Ok(Self::Hour),
            "day" => Ok(Self::Day),
            "week" => Ok(Self::Week),
            "month" => Ok(Self::Month),
            "year" => Ok(Self::Year),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrieveStatsRequestInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Minute => write!(f, "minute"),
            Self::FiveMinutes => write!(f, "five_minutes"),
            Self::ThirtyMinutes => write!(f, "thirty_minutes"),
            Self::Hour => write!(f, "hour"),
            Self::Day => write!(f, "day"),
            Self::Week => write!(f, "week"),
            Self::Month => write!(f, "month"),
            Self::Year => write!(f, "year"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
