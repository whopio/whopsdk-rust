pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostChatMessageCreatedPayloadType {
    #[serde(rename = "chat.message.created")]
    ChatMessageCreated,
}
impl fmt::Display for PostChatMessageCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ChatMessageCreated => "chat.message.created",
        };
        write!(f, "{}", s)
    }
}
