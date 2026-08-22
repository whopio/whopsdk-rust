pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateQuoteSwapsResponseObject {
    #[serde(rename = "swap_quote")]
    SwapQuote,
}
impl fmt::Display for CreateQuoteSwapsResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SwapQuote => "swap_quote",
        };
        write!(f, "{}", s)
    }
}
