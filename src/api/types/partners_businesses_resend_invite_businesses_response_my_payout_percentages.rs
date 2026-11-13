pub use crate::prelude::*;

/// The caller's own cut of each income source on this business, as a fraction of the same base as `payout_percentages` (0.12 = 12%). A solo partner's cut is the whole rate. On a referring partner business's deal it is the caller's team share of the rate: a closer earns their share, a manager earns their share minus the share of the person below them, and a deal participant earns their cut. Null when the caller earns nothing on this business, as with an account API key.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResendInviteBusinessesResponseMyPayoutPercentages {
    /// The caller's share of the referred business's Whop Ads spend.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ad_spend: Option<f64>,
    /// The caller's share of Whop's profit from card interchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub card_interchange: Option<f64>,
    /// The caller's share of Whop's profit from product sales.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub sales: Option<f64>,
    /// The caller's share of Whop's profit from platform balance transfers.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub transfer: Option<f64>,
    /// The caller's share of Whop's profit from withdrawals.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub withdrawal: Option<f64>,
}

impl ResendInviteBusinessesResponseMyPayoutPercentages {
    pub fn builder() -> ResendInviteBusinessesResponseMyPayoutPercentagesBuilder {
        <ResendInviteBusinessesResponseMyPayoutPercentagesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResendInviteBusinessesResponseMyPayoutPercentagesBuilder {
    ad_spend: Option<f64>,
    card_interchange: Option<f64>,
    sales: Option<f64>,
    transfer: Option<f64>,
    withdrawal: Option<f64>,
}

impl ResendInviteBusinessesResponseMyPayoutPercentagesBuilder {
    pub fn ad_spend(mut self, value: f64) -> Self {
        self.ad_spend = Some(value);
        self
    }

    pub fn card_interchange(mut self, value: f64) -> Self {
        self.card_interchange = Some(value);
        self
    }

    pub fn sales(mut self, value: f64) -> Self {
        self.sales = Some(value);
        self
    }

    pub fn transfer(mut self, value: f64) -> Self {
        self.transfer = Some(value);
        self
    }

    pub fn withdrawal(mut self, value: f64) -> Self {
        self.withdrawal = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResendInviteBusinessesResponseMyPayoutPercentages`].
    pub fn build(self) -> Result<ResendInviteBusinessesResponseMyPayoutPercentages, BuildError> {
        Ok(ResendInviteBusinessesResponseMyPayoutPercentages {
            ad_spend: self.ad_spend,
            card_interchange: self.card_interchange,
            sales: self.sales,
            transfer: self.transfer,
            withdrawal: self.withdrawal,
        })
    }
}
