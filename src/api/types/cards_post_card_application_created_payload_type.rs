pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostCardApplicationCreatedPayloadType {
    #[serde(rename = "card_application.created")]
    CardApplicationCreated,
}
impl fmt::Display for PostCardApplicationCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CardApplicationCreated => "card_application.created",
        };
        write!(f, "{}", s)
    }
}
