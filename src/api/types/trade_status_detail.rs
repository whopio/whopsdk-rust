pub use crate::prelude::*;

/// `partial_fill` when the order filled only part of its size. `null` otherwise.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TradeStatusDetail {
    #[serde(rename = "partial_fill")]
    PartialFill,
}
impl fmt::Display for TradeStatusDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PartialFill => "partial_fill",
        };
        write!(f, "{}", s)
    }
}
