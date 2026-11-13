pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TradingPositionObject {
    #[serde(rename = "trading_position")]
    TradingPosition,
}
impl fmt::Display for TradingPositionObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::TradingPosition => "trading_position",
        };
        write!(f, "{}", s)
    }
}
