pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LedgerActivityResourceOwnerOwnerLogoUrlObject {
    #[serde(rename = "account")]
    Account,
}
impl fmt::Display for LedgerActivityResourceOwnerOwnerLogoUrlObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Account => "account",
        };
        write!(f, "{}", s)
    }
}
