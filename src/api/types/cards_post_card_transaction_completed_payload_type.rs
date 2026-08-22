pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostCardTransactionCompletedPayloadType {
    #[serde(rename = "card_transaction.completed")]
    CardTransactionCompleted,
}
impl fmt::Display for PostCardTransactionCompletedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CardTransactionCompleted => "card_transaction.completed",
        };
        write!(f, "{}", s)
    }
}
