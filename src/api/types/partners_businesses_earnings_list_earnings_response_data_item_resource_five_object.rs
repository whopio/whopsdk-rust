pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListEarningsResponseDataItemResourceFiveObject {
    #[serde(rename = "partner_reward")]
    PartnerReward,
}
impl fmt::Display for ListEarningsResponseDataItemResourceFiveObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PartnerReward => "partner_reward",
        };
        write!(f, "{}", s)
    }
}
