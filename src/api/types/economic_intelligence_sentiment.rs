pub use crate::prelude::*;

/// How the user rated this recommendation, or `null` if they have not rated it
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EconomicIntelligenceSentiment {
    Positive,
    Negative,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EconomicIntelligenceSentiment {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Positive => serializer.serialize_str("positive"),
            Self::Negative => serializer.serialize_str("negative"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EconomicIntelligenceSentiment {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "positive" => Ok(Self::Positive),
            "negative" => Ok(Self::Negative),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EconomicIntelligenceSentiment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Positive => write!(f, "positive"),
            Self::Negative => write!(f, "negative"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
