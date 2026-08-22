pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostCardTransactionReversedPayloadType {
    #[serde(rename = "card_transaction.reversed")]
    CardTransactionReversed,
}
impl fmt::Display for PostCardTransactionReversedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CardTransactionReversed => "card_transaction.reversed",
        };
        write!(f, "{}", s)
    }
}
