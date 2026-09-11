pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExperimentResourceReferenceObject {
    App,
    AppBuild,
    Product,
    Plan,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ExperimentResourceReferenceObject {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::App => serializer.serialize_str("app"),
            Self::AppBuild => serializer.serialize_str("app_build"),
            Self::Product => serializer.serialize_str("product"),
            Self::Plan => serializer.serialize_str("plan"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ExperimentResourceReferenceObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "app" => Ok(Self::App),
            "app_build" => Ok(Self::AppBuild),
            "product" => Ok(Self::Product),
            "plan" => Ok(Self::Plan),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ExperimentResourceReferenceObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::App => write!(f, "app"),
            Self::AppBuild => write!(f, "app_build"),
            Self::Product => write!(f, "product"),
            Self::Plan => write!(f, "plan"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
