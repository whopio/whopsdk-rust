pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostChatReactionCreatedPayloadType {
    #[serde(rename = "chat.reaction.created")]
    ChatReactionCreated,
}
impl fmt::Display for PostChatReactionCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ChatReactionCreated => "chat.reaction.created",
        };
        write!(f, "{}", s)
    }
}
