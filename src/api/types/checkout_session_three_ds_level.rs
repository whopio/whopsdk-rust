pub use crate::prelude::*;

/// The 3D Secure behavior the checkout configuration asks for, or `null` to use the plan's or the account's default.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckoutSessionThreeDsLevel {
    MandateChallenge,
    Frictionless,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CheckoutSessionThreeDsLevel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MandateChallenge => serializer.serialize_str("mandate_challenge"),
            Self::Frictionless => serializer.serialize_str("frictionless"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CheckoutSessionThreeDsLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mandate_challenge" => Ok(Self::MandateChallenge),
            "frictionless" => Ok(Self::Frictionless),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CheckoutSessionThreeDsLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MandateChallenge => write!(f, "mandate_challenge"),
            Self::Frictionless => write!(f, "frictionless"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
