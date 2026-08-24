pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostPaymentSucceededPayloadType {
    #[serde(rename = "payment.succeeded")]
    PaymentSucceeded,
}
impl fmt::Display for PostPaymentSucceededPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PaymentSucceeded => "payment.succeeded",
        };
        write!(f, "{}", s)
    }
}
