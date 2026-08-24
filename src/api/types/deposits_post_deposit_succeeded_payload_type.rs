pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostDepositSucceededPayloadType {
    #[serde(rename = "deposit.succeeded")]
    DepositSucceeded,
}
impl fmt::Display for PostDepositSucceededPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DepositSucceeded => "deposit.succeeded",
        };
        write!(f, "{}", s)
    }
}
