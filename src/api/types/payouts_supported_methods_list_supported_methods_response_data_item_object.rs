pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListSupportedMethodsResponseDataItemObject {
    #[serde(rename = "supported_payout_method")]
    SupportedPayoutMethod,
}
impl fmt::Display for ListSupportedMethodsResponseDataItemObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SupportedPayoutMethod => "supported_payout_method",
        };
        write!(f, "{}", s)
    }
}
