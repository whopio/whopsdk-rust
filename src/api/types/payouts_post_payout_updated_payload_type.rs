pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostPayoutUpdatedPayloadType {
    #[serde(rename = "payout.updated")]
    PayoutUpdated,
}
impl fmt::Display for PostPayoutUpdatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PayoutUpdated => "payout.updated",
        };
        write!(f, "{}", s)
    }
}
