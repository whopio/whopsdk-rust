pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostPaymentAuthorizedPayloadType {
    #[serde(rename = "payment.authorized")]
    PaymentAuthorized,
}
impl fmt::Display for PostPaymentAuthorizedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PaymentAuthorized => "payment.authorized",
        };
        write!(f, "{}", s)
    }
}
