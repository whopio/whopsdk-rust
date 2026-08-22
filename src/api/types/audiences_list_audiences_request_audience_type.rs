pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListAudiencesRequestAudienceType {
    Custom,
    Lookalike,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListAudiencesRequestAudienceType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Custom => serializer.serialize_str("custom"),
            Self::Lookalike => serializer.serialize_str("lookalike"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListAudiencesRequestAudienceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "custom" => Ok(Self::Custom),
            "lookalike" => Ok(Self::Lookalike),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListAudiencesRequestAudienceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom => write!(f, "custom"),
            Self::Lookalike => write!(f, "lookalike"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
