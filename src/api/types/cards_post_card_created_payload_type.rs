pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostCardCreatedPayloadType {
    #[serde(rename = "card.created")]
    CardCreated,
}
impl fmt::Display for PostCardCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CardCreated => "card.created",
        };
        write!(f, "{}", s)
    }
}
