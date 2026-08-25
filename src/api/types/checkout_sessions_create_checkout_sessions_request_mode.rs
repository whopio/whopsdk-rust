pub use crate::prelude::*;

/// Defaults to the checkout configuration's mode, then `payment`. `setup` sessions are not yet available and are refused.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateCheckoutSessionsRequestMode {
    Payment,
    Setup,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateCheckoutSessionsRequestMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Payment => serializer.serialize_str("payment"),
            Self::Setup => serializer.serialize_str("setup"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateCheckoutSessionsRequestMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "payment" => Ok(Self::Payment),
            "setup" => Ok(Self::Setup),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateCheckoutSessionsRequestMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Payment => write!(f, "payment"),
            Self::Setup => write!(f, "setup"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
