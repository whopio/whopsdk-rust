pub use crate::prelude::*;

/// The available bounty implementation types.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BountyTypes {
    Classic,
    UserFunded,
    Workforce,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BountyTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Classic => serializer.serialize_str("classic"),
            Self::UserFunded => serializer.serialize_str("user_funded"),
            Self::Workforce => serializer.serialize_str("workforce"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BountyTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "classic" => Ok(Self::Classic),
            "user_funded" => Ok(Self::UserFunded),
            "workforce" => Ok(Self::Workforce),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BountyTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Classic => write!(f, "classic"),
            Self::UserFunded => write!(f, "user_funded"),
            Self::Workforce => write!(f, "workforce"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
