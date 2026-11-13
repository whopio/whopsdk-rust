pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TradingAccountObject {
    #[serde(rename = "trading_account")]
    TradingAccount,
}
impl fmt::Display for TradingAccountObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::TradingAccount => "trading_account",
        };
        write!(f, "{}", s)
    }
}
