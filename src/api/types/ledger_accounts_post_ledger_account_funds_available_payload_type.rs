pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostLedgerAccountFundsAvailablePayloadType {
    #[serde(rename = "ledger_account.funds_available")]
    LedgerAccountFundsAvailable,
}
impl fmt::Display for PostLedgerAccountFundsAvailablePayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::LedgerAccountFundsAvailable => "ledger_account.funds_available",
        };
        write!(f, "{}", s)
    }
}
