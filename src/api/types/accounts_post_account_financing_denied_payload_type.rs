pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostAccountFinancingDeniedPayloadType {
    #[serde(rename = "account.financing_denied")]
    AccountFinancingDenied,
}
impl fmt::Display for PostAccountFinancingDeniedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AccountFinancingDenied => "account.financing_denied",
        };
        write!(f, "{}", s)
    }
}
