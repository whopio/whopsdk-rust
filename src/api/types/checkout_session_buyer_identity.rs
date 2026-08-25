pub use crate::prelude::*;

/// How well this checkout knows its buyer, or `null` before one is resolved. `attributed` means an account was matched from what the buyer typed — it says who the purchase is for and nothing about who is at the keyboard. `authenticated` means the person proved they hold that account during this checkout. Only `authenticated` may be handed anything that acts as the buyer, and the value only strengthens.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckoutSessionBuyerIdentity {
    Attributed,
    Authenticated,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CheckoutSessionBuyerIdentity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Attributed => serializer.serialize_str("attributed"),
            Self::Authenticated => serializer.serialize_str("authenticated"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CheckoutSessionBuyerIdentity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "attributed" => Ok(Self::Attributed),
            "authenticated" => Ok(Self::Authenticated),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CheckoutSessionBuyerIdentity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Attributed => write!(f, "attributed"),
            Self::Authenticated => write!(f, "authenticated"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
