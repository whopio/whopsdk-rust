pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostAccountFinancingDisabledPayloadType {
    #[serde(rename = "account.financing_disabled")]
    AccountFinancingDisabled,
}
impl fmt::Display for PostAccountFinancingDisabledPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AccountFinancingDisabled => "account.financing_disabled",
        };
        write!(f, "{}", s)
    }
}
