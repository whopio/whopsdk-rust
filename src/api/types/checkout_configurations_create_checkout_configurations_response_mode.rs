pub use crate::prelude::*;

/// Controls whether checkout charges the buyer immediately or saves payment details for later.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateCheckoutConfigurationsResponseMode {
    Payment,
    Setup,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateCheckoutConfigurationsResponseMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Payment => serializer.serialize_str("payment"),
            Self::Setup => serializer.serialize_str("setup"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateCheckoutConfigurationsResponseMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "payment" => Ok(Self::Payment),
            "setup" => Ok(Self::Setup),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateCheckoutConfigurationsResponseMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Payment => write!(f, "payment"),
            Self::Setup => write!(f, "setup"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
