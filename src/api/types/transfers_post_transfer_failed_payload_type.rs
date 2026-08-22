pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostTransferFailedPayloadType {
    #[serde(rename = "transfer.failed")]
    TransferFailed,
}
impl fmt::Display for PostTransferFailedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::TransferFailed => "transfer.failed",
        };
        write!(f, "{}", s)
    }
}
