pub use crate::prelude::*;

/// The newly funded link is ready to claim.
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
