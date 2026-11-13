pub use crate::prelude::*;

/// Trading venue that holds the positions and orders.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TradingAccountProvider {
    #[serde(rename = "hyperliquid")]
    Hyperliquid,
}
impl fmt::Display for TradingAccountProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Hyperliquid => "hyperliquid",
        };
        write!(f, "{}", s)
    }
}
