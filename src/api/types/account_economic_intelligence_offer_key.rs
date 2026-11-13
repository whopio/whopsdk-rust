pub use crate::prelude::*;

/// The unique identifier for this offer. Pass this value as `economic_intelligence_duration_key` to turn it on. Only `3_days`, the auto-renew offer, is offered. The other keys identify periods bought before it became the only plan.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountEconomicIntelligenceOfferKey {
    ThreeDays,
    Weekly,
    SevenDays,
    OneDay,
    OneHour,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountEconomicIntelligenceOfferKey {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ThreeDays => serializer.serialize_str("3_days"),
            Self::Weekly => serializer.serialize_str("weekly"),
            Self::SevenDays => serializer.serialize_str("7_days"),
            Self::OneDay => serializer.serialize_str("1_day"),
            Self::OneHour => serializer.serialize_str("1_hour"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountEconomicIntelligenceOfferKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "3_days" => Ok(Self::ThreeDays),
            "weekly" => Ok(Self::Weekly),
            "7_days" => Ok(Self::SevenDays),
            "1_day" => Ok(Self::OneDay),
            "1_hour" => Ok(Self::OneHour),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountEconomicIntelligenceOfferKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ThreeDays => write!(f, "3_days"),
            Self::Weekly => write!(f, "weekly"),
            Self::SevenDays => write!(f, "7_days"),
            Self::OneDay => write!(f, "1_day"),
            Self::OneHour => write!(f, "1_hour"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
