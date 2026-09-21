pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListEarningsResponseDataItemResourceCreatedAtObject {
    #[serde(rename = "partner_reward")]
    PartnerReward,
}
impl fmt::Display for ListEarningsResponseDataItemResourceCreatedAtObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PartnerReward => "partner_reward",
        };
        write!(f, "{}", s)
    }
}
