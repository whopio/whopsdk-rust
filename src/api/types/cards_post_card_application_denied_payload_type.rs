pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostCardApplicationDeniedPayloadType {
    #[serde(rename = "card_application.denied")]
    CardApplicationDenied,
}
impl fmt::Display for PostCardApplicationDeniedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CardApplicationDenied => "card_application.denied",
        };
        write!(f, "{}", s)
    }
}
