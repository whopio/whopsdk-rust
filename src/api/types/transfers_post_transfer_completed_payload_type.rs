pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostTransferCompletedPayloadType {
    #[serde(rename = "transfer.completed")]
    TransferCompleted,
}
impl fmt::Display for PostTransferCompletedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::TransferCompleted => "transfer.completed",
        };
        write!(f, "{}", s)
    }
}
