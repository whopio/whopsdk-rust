pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostWithdrawalReversedPayloadType {
    #[serde(rename = "withdrawal.reversed")]
    WithdrawalReversed,
}
impl fmt::Display for PostWithdrawalReversedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::WithdrawalReversed => "withdrawal.reversed",
        };
        write!(f, "{}", s)
    }
}
