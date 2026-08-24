pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostDisputeUpdatedPayloadType {
    #[serde(rename = "dispute.updated")]
    DisputeUpdated,
}
impl fmt::Display for PostDisputeUpdatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DisputeUpdated => "dispute.updated",
        };
        write!(f, "{}", s)
    }
}
