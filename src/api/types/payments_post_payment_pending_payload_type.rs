pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostPaymentPendingPayloadType {
    #[serde(rename = "payment.pending")]
    PaymentPending,
}
impl fmt::Display for PostPaymentPendingPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PaymentPending => "payment.pending",
        };
        write!(f, "{}", s)
    }
}
