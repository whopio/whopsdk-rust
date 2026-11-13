pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountEconomicIntelligenceOffer {
    /// Whether this offer renews every period until auto-renew is turned off, rather than ending after one.
    #[serde(default)]
    pub auto_renew: bool,
    /// The length of each period Economic Intelligence renews for, in `duration_unit`, such as `3` for 3 days.
    #[serde(default)]
    pub duration: i64,
    /// The unit of time the duration is in (hours or days)
    pub duration_unit: AccountEconomicIntelligenceOfferDurationUnit,
    /// Percentage of payment volume charged while Economic Intelligence is on, such as `3` for 3%.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub fee_percentage: f64,
    /// The unique identifier for this offer. Pass this value as `economic_intelligence_duration_key` to turn it on. Only `3_days`, the auto-renew offer, is offered. The other keys identify periods bought before it became the only plan.
    pub key: AccountEconomicIntelligenceOfferKey,
    /// Whether Whop recommends this offer. At most one offer is recommended.
    #[serde(default)]
    pub recommended: bool,
}

impl AccountEconomicIntelligenceOffer {
    pub fn builder() -> AccountEconomicIntelligenceOfferBuilder {
        <AccountEconomicIntelligenceOfferBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountEconomicIntelligenceOfferBuilder {
    auto_renew: Option<bool>,
    duration: Option<i64>,
    duration_unit: Option<AccountEconomicIntelligenceOfferDurationUnit>,
    fee_percentage: Option<f64>,
    key: Option<AccountEconomicIntelligenceOfferKey>,
    recommended: Option<bool>,
}

impl AccountEconomicIntelligenceOfferBuilder {
    pub fn auto_renew(mut self, value: bool) -> Self {
        self.auto_renew = Some(value);
        self
    }

    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn duration_unit(mut self, value: AccountEconomicIntelligenceOfferDurationUnit) -> Self {
        self.duration_unit = Some(value);
        self
    }

    pub fn fee_percentage(mut self, value: f64) -> Self {
        self.fee_percentage = Some(value);
        self
    }

    pub fn key(mut self, value: AccountEconomicIntelligenceOfferKey) -> Self {
        self.key = Some(value);
        self
    }

    pub fn recommended(mut self, value: bool) -> Self {
        self.recommended = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountEconomicIntelligenceOffer`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auto_renew`](AccountEconomicIntelligenceOfferBuilder::auto_renew)
    /// - [`duration`](AccountEconomicIntelligenceOfferBuilder::duration)
    /// - [`duration_unit`](AccountEconomicIntelligenceOfferBuilder::duration_unit)
    /// - [`fee_percentage`](AccountEconomicIntelligenceOfferBuilder::fee_percentage)
    /// - [`key`](AccountEconomicIntelligenceOfferBuilder::key)
    /// - [`recommended`](AccountEconomicIntelligenceOfferBuilder::recommended)
    pub fn build(self) -> Result<AccountEconomicIntelligenceOffer, BuildError> {
        Ok(AccountEconomicIntelligenceOffer {
            auto_renew: self
                .auto_renew
                .ok_or_else(|| BuildError::missing_field("auto_renew"))?,
            duration: self
                .duration
                .ok_or_else(|| BuildError::missing_field("duration"))?,
            duration_unit: self
                .duration_unit
                .ok_or_else(|| BuildError::missing_field("duration_unit"))?,
            fee_percentage: self
                .fee_percentage
                .ok_or_else(|| BuildError::missing_field("fee_percentage"))?,
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            recommended: self
                .recommended
                .ok_or_else(|| BuildError::missing_field("recommended"))?,
        })
    }
}
