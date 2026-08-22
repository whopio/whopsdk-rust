pub use crate::prelude::*;

/// The different platforms an app build can target.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppBuildPlatforms {
    Ios,
    Android,
    Web,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AppBuildPlatforms {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ios => serializer.serialize_str("ios"),
            Self::Android => serializer.serialize_str("android"),
            Self::Web => serializer.serialize_str("web"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AppBuildPlatforms {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ios" => Ok(Self::Ios),
            "android" => Ok(Self::Android),
            "web" => Ok(Self::Web),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AppBuildPlatforms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ios => write!(f, "ios"),
            Self::Android => write!(f, "android"),
            Self::Web => write!(f, "web"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
