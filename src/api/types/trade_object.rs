pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TradeObject {
    #[serde(rename = "trade")]
    Trade,
}
impl fmt::Display for TradeObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Trade => "trade",
        };
        write!(f, "{}", s)
    }
}
