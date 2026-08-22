pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostPayoutMethodCreatedPayloadType {
    #[serde(rename = "payout_method.created")]
    PayoutMethodCreated,
}
impl fmt::Display for PostPayoutMethodCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PayoutMethodCreated => "payout_method.created",
        };
        write!(f, "{}", s)
    }
}
