pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostFinancialActivityFundsAvailablePayloadType {
    #[serde(rename = "financial_activity.funds_available")]
    FinancialActivityFundsAvailable,
}
impl fmt::Display for PostFinancialActivityFundsAvailablePayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::FinancialActivityFundsAvailable => "financial_activity.funds_available",
        };
        write!(f, "{}", s)
    }
}
