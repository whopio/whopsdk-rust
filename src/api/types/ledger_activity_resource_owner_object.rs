pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LedgerActivityResourceOwnerObject {
    #[serde(rename = "ledger_account")]
    LedgerAccount,
}
impl fmt::Display for LedgerActivityResourceOwnerObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::LedgerAccount => "ledger_account",
        };
        write!(f, "{}", s)
    }
}
