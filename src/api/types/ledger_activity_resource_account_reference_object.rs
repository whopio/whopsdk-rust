pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LedgerActivityResourceAccountReferenceObject {
    #[serde(rename = "payout_method")]
    PayoutMethod,
}
impl fmt::Display for LedgerActivityResourceAccountReferenceObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PayoutMethod => "payout_method",
        };
        write!(f, "{}", s)
    }
}
