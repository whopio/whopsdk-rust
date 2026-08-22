pub use crate::prelude::*;

/// The type of plan that can be attached to a product
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlanTypes {
    Renewal,
    OneTime,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PlanTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Renewal => serializer.serialize_str("renewal"),
            Self::OneTime => serializer.serialize_str("one_time"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PlanTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "renewal" => Ok(Self::Renewal),
            "one_time" => Ok(Self::OneTime),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PlanTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Renewal => write!(f, "renewal"),
            Self::OneTime => write!(f, "one_time"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
