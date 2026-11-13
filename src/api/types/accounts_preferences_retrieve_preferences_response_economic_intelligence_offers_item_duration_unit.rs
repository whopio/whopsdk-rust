pub use crate::prelude::*;

/// The unit of time the duration is in (hours or days)
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrievePreferencesResponseEconomicIntelligenceOffersItemDurationUnit {
    Hours,
    Days,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrievePreferencesResponseEconomicIntelligenceOffersItemDurationUnit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Hours => serializer.serialize_str("hours"),
            Self::Days => serializer.serialize_str("days"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for RetrievePreferencesResponseEconomicIntelligenceOffersItemDurationUnit
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "hours" => Ok(Self::Hours),
            "days" => Ok(Self::Days),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrievePreferencesResponseEconomicIntelligenceOffersItemDurationUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hours => write!(f, "hours"),
            Self::Days => write!(f, "days"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
