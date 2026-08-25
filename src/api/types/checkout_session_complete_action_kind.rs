pub use crate::prelude::*;

/// What the ceremony finishes: `payment` for a charge with a step left, `setup` for a payment method still being saved — a paid waitlist vaults the card without charging it. Informational: the executor is the same either way, so run the secret regardless of a `kind` you do not recognize.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckoutSessionCompleteActionKind {
    Payment,
    Setup,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CheckoutSessionCompleteActionKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Payment => serializer.serialize_str("payment"),
            Self::Setup => serializer.serialize_str("setup"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CheckoutSessionCompleteActionKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "payment" => Ok(Self::Payment),
            "setup" => Ok(Self::Setup),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CheckoutSessionCompleteActionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Payment => write!(f, "payment"),
            Self::Setup => write!(f, "setup"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
