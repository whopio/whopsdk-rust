pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostPaymentCanceledPayloadType {
    #[serde(rename = "payment.canceled")]
    PaymentCanceled,
}
impl fmt::Display for PostPaymentCanceledPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PaymentCanceled => "payment.canceled",
        };
        write!(f, "{}", s)
    }
}
