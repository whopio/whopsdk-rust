pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostWithdrawalUpdatedPayloadType {
    #[serde(rename = "withdrawal.updated")]
    WithdrawalUpdated,
}
impl fmt::Display for PostWithdrawalUpdatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::WithdrawalUpdated => "withdrawal.updated",
        };
        write!(f, "{}", s)
    }
}
