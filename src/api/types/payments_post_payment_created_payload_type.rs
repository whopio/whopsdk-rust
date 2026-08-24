pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostPaymentCreatedPayloadType {
    #[serde(rename = "payment.created")]
    PaymentCreated,
}
impl fmt::Display for PostPaymentCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PaymentCreated => "payment.created",
        };
        write!(f, "{}", s)
    }
}
