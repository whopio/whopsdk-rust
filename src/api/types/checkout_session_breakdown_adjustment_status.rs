pub use crate::prelude::*;

/// Present only on an adjustment with no figure yet — render its row in a loading state and expect `calculate_breakdown` to resolve it. An adjustment that does not apply is absent from the list entirely, never a zero row.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CheckoutSessionBreakdownAdjustmentStatus {
    #[serde(rename = "pending")]
    Pending,
}
impl fmt::Display for CheckoutSessionBreakdownAdjustmentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Pending => "pending",
        };
        write!(f, "{}", s)
    }
}
