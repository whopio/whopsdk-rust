pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostPaymentFailedPayloadType {
    #[serde(rename = "payment.failed")]
    PaymentFailed,
}
impl fmt::Display for PostPaymentFailedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PaymentFailed => "payment.failed",
        };
        write!(f, "{}", s)
    }
}
