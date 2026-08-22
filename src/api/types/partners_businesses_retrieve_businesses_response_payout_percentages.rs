pub use crate::prelude::*;

/// The referrer's commission rate for each income source, expressed as a fraction (0.3 = 30%).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RetrieveBusinessesResponsePayoutPercentages {
    /// Share of the referred business's Whop Ads spend.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ad_spend: Option<f64>,
    /// Share of Whop's profit from card interchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub card_interchange: Option<f64>,
    /// Share of Whop's profit from product sales.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub sales: f64,
    /// Share of Whop's profit from platform balance transfers.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub transfer: Option<f64>,
}

impl RetrieveBusinessesResponsePayoutPercentages {
    pub fn builder() -> RetrieveBusinessesResponsePayoutPercentagesBuilder {
        <RetrieveBusinessesResponsePayoutPercentagesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveBusinessesResponsePayoutPercentagesBuilder {
    ad_spend: Option<f64>,
    card_interchange: Option<f64>,
    sales: Option<f64>,
    transfer: Option<f64>,
}

impl RetrieveBusinessesResponsePayoutPercentagesBuilder {
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

    /// Consumes the builder and constructs a [`RetrieveBusinessesResponsePayoutPercentages`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sales`](RetrieveBusinessesResponsePayoutPercentagesBuilder::sales)
    pub fn build(self) -> Result<RetrieveBusinessesResponsePayoutPercentages, BuildError> {
        Ok(RetrieveBusinessesResponsePayoutPercentages {
            ad_spend: self.ad_spend,
            card_interchange: self.card_interchange,
            sales: self
                .sales
                .ok_or_else(|| BuildError::missing_field("sales"))?,
            transfer: self.transfer,
        })
    }
}
