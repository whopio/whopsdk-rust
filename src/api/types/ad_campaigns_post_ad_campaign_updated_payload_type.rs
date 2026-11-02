pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostAdCampaignUpdatedPayloadType {
    #[serde(rename = "ad_campaign.updated")]
    AdCampaignUpdated,
}
impl fmt::Display for PostAdCampaignUpdatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AdCampaignUpdated => "ad_campaign.updated",
        };
        write!(f, "{}", s)
    }
}
