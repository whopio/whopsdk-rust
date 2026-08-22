pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostDisputeCreatedPayloadType {
    #[serde(rename = "dispute.created")]
    DisputeCreated,
}
impl fmt::Display for PostDisputeCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DisputeCreated => "dispute.created",
        };
        write!(f, "{}", s)
    }
}
