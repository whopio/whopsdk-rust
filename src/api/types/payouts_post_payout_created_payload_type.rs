pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostPayoutCreatedPayloadType {
    #[serde(rename = "payout.created")]
    PayoutCreated,
}
impl fmt::Display for PostPayoutCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PayoutCreated => "payout.created",
        };
        write!(f, "{}", s)
    }
}
