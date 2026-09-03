pub use crate::prelude::*;

/// Why instant delivery is unavailable for this method. `minimum_crypto_sales_not_met` means the account has not reached the total sales required for instant cryptocurrency payouts. `null` when this restriction does not apply.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListMethodsResponseDataItemQuoteInstantUnavailableReason {
    #[serde(rename = "minimum_crypto_sales_not_met")]
    MinimumCryptoSalesNotMet,
}
impl fmt::Display for ListMethodsResponseDataItemQuoteInstantUnavailableReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MinimumCryptoSalesNotMet => "minimum_crypto_sales_not_met",
        };
        write!(f, "{}", s)
    }
}
