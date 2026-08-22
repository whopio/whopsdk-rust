pub use crate::prelude::*;

/// The different types an product can be. Only use 'regular'. The rest are for internal use
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccessPassTypes {
    Regular,
    App,
    ExperienceUpsell,
    ApiOnly,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccessPassTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Regular => serializer.serialize_str("regular"),
            Self::App => serializer.serialize_str("app"),
            Self::ExperienceUpsell => serializer.serialize_str("experience_upsell"),
            Self::ApiOnly => serializer.serialize_str("api_only"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccessPassTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "regular" => Ok(Self::Regular),
            "app" => Ok(Self::App),
            "experience_upsell" => Ok(Self::ExperienceUpsell),
            "api_only" => Ok(Self::ApiOnly),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccessPassTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Regular => write!(f, "regular"),
            Self::App => write!(f, "app"),
            Self::ExperienceUpsell => write!(f, "experience_upsell"),
            Self::ApiOnly => write!(f, "api_only"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
