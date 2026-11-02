pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListPartnerReferralRequestsRequestOrder {
    #[serde(rename = "created_at")]
    CreatedAt,
}
impl fmt::Display for ListPartnerReferralRequestsRequestOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CreatedAt => "created_at",
        };
        write!(f, "{}", s)
    }
}
