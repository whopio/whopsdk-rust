pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdatePreferencesResponseEconomicIntelligenceOffersItem {
    /// Whether this offer renews every period until auto-renew is turned off, rather than ending after one.
    #[serde(default)]
    pub auto_renew: bool,
    /// The length of each period Economic Intelligence renews for, in `duration_unit`, such as `3` for 3 days.
    #[serde(default)]
    pub duration: i64,
    /// The unit of time the duration is in (hours or days)
    pub duration_unit: UpdatePreferencesResponseEconomicIntelligenceOffersItemDurationUnit,
    /// Percentage of payment volume charged while Economic Intelligence is on, such as `3` for 3%.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub fee_percentage: f64,
    /// The unique identifier for this offer. Pass this value as `economic_intelligence_duration_key` to turn it on. Only `3_days`, the auto-renew offer, is offered. The other keys identify periods bought before it became the only plan.
    pub key: UpdatePreferencesResponseEconomicIntelligenceOffersItemKey,
    /// Whether Whop recommends this offer. At most one offer is recommended.
    #[serde(default)]
    pub recommended: bool,
}

impl UpdatePreferencesResponseEconomicIntelligenceOffersItem {
    pub fn builder() -> UpdatePreferencesResponseEconomicIntelligenceOffersItemBuilder {
        <UpdatePreferencesResponseEconomicIntelligenceOffersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePreferencesResponseEconomicIntelligenceOffersItemBuilder {
    auto_renew: Option<bool>,
    duration: Option<i64>,
    duration_unit: Option<UpdatePreferencesResponseEconomicIntelligenceOffersItemDurationUnit>,
    fee_percentage: Option<f64>,
    key: Option<UpdatePreferencesResponseEconomicIntelligenceOffersItemKey>,
    recommended: Option<bool>,
}

impl UpdatePreferencesResponseEconomicIntelligenceOffersItemBuilder {
    pub fn auto_renew(mut self, value: bool) -> Self {
        self.auto_renew = Some(value);
        self
    }

    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn duration_unit(
        mut self,
        value: UpdatePreferencesResponseEconomicIntelligenceOffersItemDurationUnit,
    ) -> Self {
        self.duration_unit = Some(value);
        self
    }

    pub fn fee_percentage(mut self, value: f64) -> Self {
        self.fee_percentage = Some(value);
        self
    }

    pub fn key(
        mut self,
        value: UpdatePreferencesResponseEconomicIntelligenceOffersItemKey,
    ) -> Self {
        self.key = Some(value);
        self
    }

    pub fn recommended(mut self, value: bool) -> Self {
        self.recommended = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePreferencesResponseEconomicIntelligenceOffersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auto_renew`](UpdatePreferencesResponseEconomicIntelligenceOffersItemBuilder::auto_renew)
    /// - [`duration`](UpdatePreferencesResponseEconomicIntelligenceOffersItemBuilder::duration)
    /// - [`duration_unit`](UpdatePreferencesResponseEconomicIntelligenceOffersItemBuilder::duration_unit)
    /// - [`fee_percentage`](UpdatePreferencesResponseEconomicIntelligenceOffersItemBuilder::fee_percentage)
    /// - [`key`](UpdatePreferencesResponseEconomicIntelligenceOffersItemBuilder::key)
    /// - [`recommended`](UpdatePreferencesResponseEconomicIntelligenceOffersItemBuilder::recommended)
    pub fn build(
        self,
    ) -> Result<UpdatePreferencesResponseEconomicIntelligenceOffersItem, BuildError> {
        Ok(UpdatePreferencesResponseEconomicIntelligenceOffersItem {
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
