pub use crate::prelude::*;

/// Operating system targeted.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdGroupOperatingSystemOs {
    Ios,
    Android,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdGroupOperatingSystemOs {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ios => serializer.serialize_str("ios"),
            Self::Android => serializer.serialize_str("android"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdGroupOperatingSystemOs {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ios" => Ok(Self::Ios),
            "android" => Ok(Self::Android),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdGroupOperatingSystemOs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ios => write!(f, "ios"),
            Self::Android => write!(f, "android"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
