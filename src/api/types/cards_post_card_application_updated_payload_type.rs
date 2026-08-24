pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostCardApplicationUpdatedPayloadType {
    #[serde(rename = "card_application.updated")]
    CardApplicationUpdated,
}
impl fmt::Display for PostCardApplicationUpdatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CardApplicationUpdated => "card_application.updated",
        };
        write!(f, "{}", s)
    }
}
