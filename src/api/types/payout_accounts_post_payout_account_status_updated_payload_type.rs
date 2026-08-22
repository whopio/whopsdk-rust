pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostPayoutAccountStatusUpdatedPayloadType {
    #[serde(rename = "payout_account.status_updated")]
    PayoutAccountStatusUpdated,
}
impl fmt::Display for PostPayoutAccountStatusUpdatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PayoutAccountStatusUpdated => "payout_account.status_updated",
        };
        write!(f, "{}", s)
    }
}
