pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostCardCanceledPayloadType {
    #[serde(rename = "card.canceled")]
    CardCanceled,
}
impl fmt::Display for PostCardCanceledPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CardCanceled => "card.canceled",
        };
        write!(f, "{}", s)
    }
}
