pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListEarningsResponseDataItemResourceTwoObject {
    #[serde(rename = "withdrawal")]
    Withdrawal,
}
impl fmt::Display for ListEarningsResponseDataItemResourceTwoObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Withdrawal => "withdrawal",
        };
        write!(f, "{}", s)
    }
}
