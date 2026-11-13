pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TradingOrderObject {
    #[serde(rename = "trading_order")]
    TradingOrder,
}
impl fmt::Display for TradingOrderObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::TradingOrder => "trading_order",
        };
        write!(f, "{}", s)
    }
}
