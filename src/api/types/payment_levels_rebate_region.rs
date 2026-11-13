pub use crate::prelude::*;

/// Card region the processing fee was charged in, which sets `processing_rate_percentage`. `null` unless `status` is `rebated`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentLevelsRebateRegion {
    UsAndCanada,
    EuropeUkAndAustralia,
    LatinAmerica,
    Peru,
    Brazil,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentLevelsRebateRegion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::UsAndCanada => serializer.serialize_str("us_and_canada"),
            Self::EuropeUkAndAustralia => serializer.serialize_str("europe_uk_and_australia"),
            Self::LatinAmerica => serializer.serialize_str("latin_america"),
            Self::Peru => serializer.serialize_str("peru"),
            Self::Brazil => serializer.serialize_str("brazil"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentLevelsRebateRegion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "us_and_canada" => Ok(Self::UsAndCanada),
            "europe_uk_and_australia" => Ok(Self::EuropeUkAndAustralia),
            "latin_america" => Ok(Self::LatinAmerica),
            "peru" => Ok(Self::Peru),
            "brazil" => Ok(Self::Brazil),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentLevelsRebateRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UsAndCanada => write!(f, "us_and_canada"),
            Self::EuropeUkAndAustralia => write!(f, "europe_uk_and_australia"),
            Self::LatinAmerica => write!(f, "latin_america"),
            Self::Peru => write!(f, "peru"),
            Self::Brazil => write!(f, "brazil"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
