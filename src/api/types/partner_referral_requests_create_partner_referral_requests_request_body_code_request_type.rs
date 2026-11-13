pub use crate::prelude::*;

/// Create or retrieve a reusable referral link without requesting attribution for an existing business.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreatePartnerReferralRequestsRequestBodyCodeRequestType {
    #[serde(rename = "link")]
    Link,
}
impl fmt::Display for CreatePartnerReferralRequestsRequestBodyCodeRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Link => "link",
        };
        write!(f, "{}", s)
    }
}
