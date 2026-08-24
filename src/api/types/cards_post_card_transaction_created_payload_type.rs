pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostCardTransactionCreatedPayloadType {
    #[serde(rename = "card_transaction.created")]
    CardTransactionCreated,
}
impl fmt::Display for PostCardTransactionCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CardTransactionCreated => "card_transaction.created",
        };
        write!(f, "{}", s)
    }
}
