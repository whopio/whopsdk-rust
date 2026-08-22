pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostCardTransactionDeclinedPayloadType {
    #[serde(rename = "card_transaction.declined")]
    CardTransactionDeclined,
}
impl fmt::Display for PostCardTransactionDeclinedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CardTransactionDeclined => "card_transaction.declined",
        };
        write!(f, "{}", s)
    }
}
