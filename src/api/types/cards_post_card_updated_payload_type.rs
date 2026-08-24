pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostCardUpdatedPayloadType {
    #[serde(rename = "card.updated")]
    CardUpdated,
}
impl fmt::Display for PostCardUpdatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CardUpdated => "card.updated",
        };
        write!(f, "{}", s)
    }
}
