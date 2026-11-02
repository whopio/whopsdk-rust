pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostPayoutReversedPayloadType {
    #[serde(rename = "payout.reversed")]
    PayoutReversed,
}
impl fmt::Display for PostPayoutReversedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PayoutReversed => "payout.reversed",
        };
        write!(f, "{}", s)
    }
}
