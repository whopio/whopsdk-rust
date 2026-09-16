pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostPaymentRequiresActionPayloadType {
    #[serde(rename = "payment.requires_action")]
    PaymentRequiresAction,
}
impl fmt::Display for PostPaymentRequiresActionPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PaymentRequiresAction => "payment.requires_action",
        };
        write!(f, "{}", s)
    }
}
