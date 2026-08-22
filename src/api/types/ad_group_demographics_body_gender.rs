pub use crate::prelude::*;

/// Gender to target.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdGroupDemographicsBodyGender {
    All,
    Male,
    Female,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdGroupDemographicsBodyGender {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::All => serializer.serialize_str("all"),
            Self::Male => serializer.serialize_str("male"),
            Self::Female => serializer.serialize_str("female"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdGroupDemographicsBodyGender {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "all" => Ok(Self::All),
            "male" => Ok(Self::Male),
            "female" => Ok(Self::Female),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdGroupDemographicsBodyGender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Male => write!(f, "male"),
            Self::Female => write!(f, "female"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
