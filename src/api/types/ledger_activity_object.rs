pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LedgerActivityObject {
    #[serde(rename = "ledger_activity")]
    LedgerActivity,
}
impl fmt::Display for LedgerActivityObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::LedgerActivity => "ledger_activity",
        };
        write!(f, "{}", s)
    }
}
