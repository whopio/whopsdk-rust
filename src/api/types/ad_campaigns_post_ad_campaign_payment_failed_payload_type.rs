pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostAdCampaignPaymentFailedPayloadType {
    #[serde(rename = "ad_campaign.payment_failed")]
    AdCampaignPaymentFailed,
}
impl fmt::Display for PostAdCampaignPaymentFailedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AdCampaignPaymentFailed => "ad_campaign.payment_failed",
        };
        write!(f, "{}", s)
    }
}
