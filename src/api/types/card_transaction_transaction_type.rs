pub use crate::prelude::*;

/// The kind of card transaction. Always `spend` today.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CardTransactionTransactionType {
    #[serde(rename = "spend")]
    Spend,
}
impl fmt::Display for CardTransactionTransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Spend => "spend",
        };
        write!(f, "{}", s)
    }
}
