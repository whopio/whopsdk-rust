pub use crate::prelude::*;

/// What the saved payment method will pay for. Set to `ads_billing` when saving a card to pay for Whop Ads on `account_id`: the card is verified by Whop Ads, the merchant that charges it, which helps minimize security declines on ad payments, and it requires `ad_campaign:create` on `account_id`. Omit it for any other payment method.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateSetupIntentsRequestPurpose {
    #[serde(rename = "ads_billing")]
    AdsBilling,
}
impl fmt::Display for CreateSetupIntentsRequestPurpose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AdsBilling => "ads_billing",
        };
        write!(f, "{}", s)
    }
}
