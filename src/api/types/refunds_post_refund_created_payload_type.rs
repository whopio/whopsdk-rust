pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostRefundCreatedPayloadType {
    #[serde(rename = "refund.created")]
    RefundCreated,
}
impl fmt::Display for PostRefundCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::RefundCreated => "refund.created",
        };
        write!(f, "{}", s)
    }
}
