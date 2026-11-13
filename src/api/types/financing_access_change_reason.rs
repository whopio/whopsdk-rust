pub use crate::prelude::*;

/// Why financing access was removed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FinancingAccessChangeReason {
    #[serde(rename = "high_dispute_rate")]
    HighDisputeRate,
}
impl fmt::Display for FinancingAccessChangeReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::HighDisputeRate => "high_dispute_rate",
        };
        write!(f, "{}", s)
    }
}
