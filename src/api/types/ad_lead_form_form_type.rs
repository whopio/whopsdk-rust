pub use crate::prelude::*;

/// `more_volume` is quickest to submit; `higher_intent` adds a confirmation step before submission.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdLeadFormFormType {
    MoreVolume,
    HigherIntent,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdLeadFormFormType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MoreVolume => serializer.serialize_str("more_volume"),
            Self::HigherIntent => serializer.serialize_str("higher_intent"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdLeadFormFormType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "more_volume" => Ok(Self::MoreVolume),
            "higher_intent" => Ok(Self::HigherIntent),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdLeadFormFormType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MoreVolume => write!(f, "more_volume"),
            Self::HigherIntent => write!(f, "higher_intent"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
