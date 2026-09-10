pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostAccountFinancingApprovedPayloadType {
    #[serde(rename = "account.financing_approved")]
    AccountFinancingApproved,
}
impl fmt::Display for PostAccountFinancingApprovedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AccountFinancingApproved => "account.financing_approved",
        };
        write!(f, "{}", s)
    }
}
