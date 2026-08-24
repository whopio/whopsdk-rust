pub use crate::prelude::*;

/// Unit for `radius`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdGroupCustomLocationDistanceUnit {
    Mile,
    Kilometer,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdGroupCustomLocationDistanceUnit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Mile => serializer.serialize_str("mile"),
            Self::Kilometer => serializer.serialize_str("kilometer"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdGroupCustomLocationDistanceUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mile" => Ok(Self::Mile),
            "kilometer" => Ok(Self::Kilometer),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdGroupCustomLocationDistanceUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mile => write!(f, "mile"),
            Self::Kilometer => write!(f, "kilometer"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
