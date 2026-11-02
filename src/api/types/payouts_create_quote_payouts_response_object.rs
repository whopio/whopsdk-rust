pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateQuotePayoutsResponseObject {
    #[serde(rename = "payout_quote")]
    PayoutQuote,
}
impl fmt::Display for CreateQuotePayoutsResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PayoutQuote => "payout_quote",
        };
        write!(f, "{}", s)
    }
}
