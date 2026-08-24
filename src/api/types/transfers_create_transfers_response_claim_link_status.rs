pub use crate::prelude::*;

/// A newly funded claim link is always `pending` — it stays claimable until it is fully claimed, canceled, or expires.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateTransfersResponseClaimLinkStatus {
    #[serde(rename = "pending")]
    Pending,
}
impl fmt::Display for CreateTransfersResponseClaimLinkStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Pending => "pending",
        };
        write!(f, "{}", s)
    }
}
