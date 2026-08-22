pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListEarningsResponseDataItemResourceCreatedAtObject {
    #[serde(rename = "card_transaction")]
    CardTransaction,
}
impl fmt::Display for ListEarningsResponseDataItemResourceCreatedAtObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CardTransaction => "card_transaction",
        };
        write!(f, "{}", s)
    }
}
