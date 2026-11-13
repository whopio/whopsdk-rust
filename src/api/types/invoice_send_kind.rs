pub use crate::prelude::*;

/// How batches are classified: manual batches use `send` when no earlier completed email history exists and `resend` otherwise. Classification happens when history is recorded. Earlier unrecorded emails are unknown. Automatic sends use `scheduled_send` for a scheduled send and `reminder` for a reminder after the due date. They use `payment_follow_up` for the payment link after a charge to the saved payment method failed or needed the customer to act.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InvoiceSendKind {
    Send,
    Resend,
    ScheduledSend,
    Reminder,
    PaymentFollowUp,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InvoiceSendKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Send => serializer.serialize_str("send"),
            Self::Resend => serializer.serialize_str("resend"),
            Self::ScheduledSend => serializer.serialize_str("scheduled_send"),
            Self::Reminder => serializer.serialize_str("reminder"),
            Self::PaymentFollowUp => serializer.serialize_str("payment_follow_up"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InvoiceSendKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "send" => Ok(Self::Send),
            "resend" => Ok(Self::Resend),
            "scheduled_send" => Ok(Self::ScheduledSend),
            "reminder" => Ok(Self::Reminder),
            "payment_follow_up" => Ok(Self::PaymentFollowUp),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InvoiceSendKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Send => write!(f, "send"),
            Self::Resend => write!(f, "resend"),
            Self::ScheduledSend => write!(f, "scheduled_send"),
            Self::Reminder => write!(f, "reminder"),
            Self::PaymentFollowUp => write!(f, "payment_follow_up"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
