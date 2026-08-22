pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LedgerActivityResourceTwoObject {
    #[serde(rename = "bounty")]
    Bounty,
}
impl fmt::Display for LedgerActivityResourceTwoObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Bounty => "bounty",
        };
        write!(f, "{}", s)
    }
}
