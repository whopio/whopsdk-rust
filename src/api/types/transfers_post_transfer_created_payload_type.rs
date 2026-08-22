pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostTransferCreatedPayloadType {
    #[serde(rename = "transfer.created")]
    TransferCreated,
}
impl fmt::Display for PostTransferCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::TransferCreated => "transfer.created",
        };
        write!(f, "{}", s)
    }
}
