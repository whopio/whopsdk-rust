pub use crate::prelude::*;

/// The type of end-user the app is built for.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppListItemAppType {
    B2BApp,
    B2CApp,
    CompanyApp,
    Component,
    Website,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AppListItemAppType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::B2BApp => serializer.serialize_str("b2b_app"),
            Self::B2CApp => serializer.serialize_str("b2c_app"),
            Self::CompanyApp => serializer.serialize_str("company_app"),
            Self::Component => serializer.serialize_str("component"),
            Self::Website => serializer.serialize_str("website"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AppListItemAppType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "b2b_app" => Ok(Self::B2BApp),
            "b2c_app" => Ok(Self::B2CApp),
            "company_app" => Ok(Self::CompanyApp),
            "component" => Ok(Self::Component),
            "website" => Ok(Self::Website),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AppListItemAppType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::B2BApp => write!(f, "b2b_app"),
            Self::B2CApp => write!(f, "b2c_app"),
            Self::CompanyApp => write!(f, "company_app"),
            Self::Component => write!(f, "component"),
            Self::Website => write!(f, "website"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
