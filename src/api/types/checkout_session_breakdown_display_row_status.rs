pub use crate::prelude::*;

/// Present only on a row whose figure is still being calculated — render it in a loading state and expect `calculate_breakdown` to resolve it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CheckoutSessionBreakdownDisplayRowStatus {
    #[serde(rename = "pending")]
    Pending,
}
impl fmt::Display for CheckoutSessionBreakdownDisplayRowStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Pending => "pending",
        };
        write!(f, "{}", s)
    }
}
