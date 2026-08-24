pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostCardApplicationApprovedPayloadType {
    #[serde(rename = "card_application.approved")]
    CardApplicationApproved,
}
impl fmt::Display for PostCardApplicationApprovedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CardApplicationApproved => "card_application.approved",
        };
        write!(f, "{}", s)
    }
}
