pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostWithdrawalCreatedPayloadType {
    #[serde(rename = "withdrawal.created")]
    WithdrawalCreated,
}
impl fmt::Display for PostWithdrawalCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::WithdrawalCreated => "withdrawal.created",
        };
        write!(f, "{}", s)
    }
}
