pub use crate::prelude::*;

/// `check_email` — the claim is not recorded yet: tell the buyer to check their email to claim their purchase. `handed_off` — the claim is recorded and the checkout finished on another device: this reader keeps seeing it, which is correct — the session never unlocks across devices.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckoutSessionAwaitClaimActionState {
    CheckEmail,
    HandedOff,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CheckoutSessionAwaitClaimActionState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CheckEmail => serializer.serialize_str("check_email"),
            Self::HandedOff => serializer.serialize_str("handed_off"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CheckoutSessionAwaitClaimActionState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "check_email" => Ok(Self::CheckEmail),
            "handed_off" => Ok(Self::HandedOff),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CheckoutSessionAwaitClaimActionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CheckEmail => write!(f, "check_email"),
            Self::HandedOff => write!(f, "handed_off"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
