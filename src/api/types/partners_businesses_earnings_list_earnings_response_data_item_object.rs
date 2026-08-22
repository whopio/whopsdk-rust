pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListEarningsResponseDataItemObject {
    #[serde(rename = "partner_business_earning")]
    PartnerBusinessEarning,
}
impl fmt::Display for ListEarningsResponseDataItemObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PartnerBusinessEarning => "partner_business_earning",
        };
        write!(f, "{}", s)
    }
}
