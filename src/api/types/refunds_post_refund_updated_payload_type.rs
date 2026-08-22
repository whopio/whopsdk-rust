pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostRefundUpdatedPayloadType {
    #[serde(rename = "refund.updated")]
    RefundUpdated,
}
impl fmt::Display for PostRefundUpdatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::RefundUpdated => "refund.updated",
        };
        write!(f, "{}", s)
    }
}
