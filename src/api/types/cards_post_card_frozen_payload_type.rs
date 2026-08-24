pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostCardFrozenPayloadType {
    #[serde(rename = "card.frozen")]
    CardFrozen,
}
impl fmt::Display for PostCardFrozenPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CardFrozen => "card.frozen",
        };
        write!(f, "{}", s)
    }
}
