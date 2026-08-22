pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostWithdrawalCreatedPayloadDataObject {
    #[serde(rename = "payout")]
    Payout,
}
impl fmt::Display for PostWithdrawalCreatedPayloadDataObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Payout => "payout",
        };
        write!(f, "{}", s)
    }
}
