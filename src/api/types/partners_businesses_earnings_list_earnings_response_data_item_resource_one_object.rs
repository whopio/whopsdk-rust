pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListEarningsResponseDataItemResourceOneObject {
    #[serde(rename = "transfer")]
    Transfer,
}
impl fmt::Display for ListEarningsResponseDataItemResourceOneObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Transfer => "transfer",
        };
        write!(f, "{}", s)
    }
}
