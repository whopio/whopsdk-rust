pub use crate::prelude::*;

/// The party that performed the action.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResolutionEventReporterType {
    Merchant,
    Customer,
    Platform,
    System,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ResolutionEventReporterType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Merchant => serializer.serialize_str("merchant"),
            Self::Customer => serializer.serialize_str("customer"),
            Self::Platform => serializer.serialize_str("platform"),
            Self::System => serializer.serialize_str("system"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ResolutionEventReporterType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "merchant" => Ok(Self::Merchant),
            "customer" => Ok(Self::Customer),
            "platform" => Ok(Self::Platform),
            "system" => Ok(Self::System),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ResolutionEventReporterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Merchant => write!(f, "merchant"),
            Self::Customer => write!(f, "customer"),
            Self::Platform => write!(f, "platform"),
            Self::System => write!(f, "system"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
