pub use crate::prelude::*;

/// The verb for the button that confirms this checkout, so every surface names the act the same way: `pay`, `subscribe`, `start_trial`, `join_waitlist`, or `continue` when nothing is charged today (a free checkout, `setup` mode saving a payment method, or a transfer that charges nothing). Render your own wording for each value — this is a key, never display text — and fall back to a generic label on a value you do not recognize.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckoutSessionCtaLabel {
    AcceptTransfer,
    Pay,
    Continue,
    StartTrial,
    Subscribe,
    JoinWaitlist,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CheckoutSessionCtaLabel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AcceptTransfer => serializer.serialize_str("accept_transfer"),
            Self::Pay => serializer.serialize_str("pay"),
            Self::Continue => serializer.serialize_str("continue"),
            Self::StartTrial => serializer.serialize_str("start_trial"),
            Self::Subscribe => serializer.serialize_str("subscribe"),
            Self::JoinWaitlist => serializer.serialize_str("join_waitlist"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CheckoutSessionCtaLabel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "accept_transfer" => Ok(Self::AcceptTransfer),
            "pay" => Ok(Self::Pay),
            "continue" => Ok(Self::Continue),
            "start_trial" => Ok(Self::StartTrial),
            "subscribe" => Ok(Self::Subscribe),
            "join_waitlist" => Ok(Self::JoinWaitlist),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CheckoutSessionCtaLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AcceptTransfer => write!(f, "accept_transfer"),
            Self::Pay => write!(f, "pay"),
            Self::Continue => write!(f, "continue"),
            Self::StartTrial => write!(f, "start_trial"),
            Self::Subscribe => write!(f, "subscribe"),
            Self::JoinWaitlist => write!(f, "join_waitlist"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
