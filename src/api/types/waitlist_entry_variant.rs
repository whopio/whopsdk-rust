pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WaitlistEntryVariant {
    /// Number of days between recurring charges, such as 30 for monthly or 365 for annual. `null` for one-time variants.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub billing_period: Option<f64>,
    /// Billing model for this variant: `one_time` or `renewal`.
    pub billing_type: WaitlistEntryVariantBillingType,
    /// Customer-visible variant description. Maximum 1000 characters. `null` if no description is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Variant ID, prefixed `plan_`.
    #[serde(default)]
    pub id: String,
    /// What checkout charges up front. `amount` is `"0.00"` when the first charge is free, such as a trial.
    #[serde(default)]
    pub initial_price: Money,
    /// The recurring charge every `billing_period` days. `amount` is `"0.00"` for one-time variants.
    #[serde(default)]
    pub renewal_price: Money,
    /// Variant display name shown to customers. `null` if no title has been set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Free trial days configured before the first renewal charge. `null` if no trial is configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub trial_period_days: Option<f64>,
}

impl WaitlistEntryVariant {
    pub fn builder() -> WaitlistEntryVariantBuilder {
        <WaitlistEntryVariantBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WaitlistEntryVariantBuilder {
    billing_period: Option<f64>,
    billing_type: Option<WaitlistEntryVariantBillingType>,
    description: Option<String>,
    id: Option<String>,
    initial_price: Option<Money>,
    renewal_price: Option<Money>,
    title: Option<String>,
    trial_period_days: Option<f64>,
}

impl WaitlistEntryVariantBuilder {
    pub fn billing_period(mut self, value: f64) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn billing_type(mut self, value: WaitlistEntryVariantBillingType) -> Self {
        self.billing_type = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn initial_price(mut self, value: Money) -> Self {
        self.initial_price = Some(value);
        self
    }

    pub fn renewal_price(mut self, value: Money) -> Self {
        self.renewal_price = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn trial_period_days(mut self, value: f64) -> Self {
        self.trial_period_days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WaitlistEntryVariant`].
    /// This method will fail if any of the following fields are not set:
    /// - [`billing_type`](WaitlistEntryVariantBuilder::billing_type)
    /// - [`id`](WaitlistEntryVariantBuilder::id)
    /// - [`initial_price`](WaitlistEntryVariantBuilder::initial_price)
    /// - [`renewal_price`](WaitlistEntryVariantBuilder::renewal_price)
    pub fn build(self) -> Result<WaitlistEntryVariant, BuildError> {
        Ok(WaitlistEntryVariant {
            billing_period: self.billing_period,
            billing_type: self
                .billing_type
                .ok_or_else(|| BuildError::missing_field("billing_type"))?,
            description: self.description,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            initial_price: self
                .initial_price
                .ok_or_else(|| BuildError::missing_field("initial_price"))?,
            renewal_price: self
                .renewal_price
                .ok_or_else(|| BuildError::missing_field("renewal_price"))?,
            title: self.title,
            trial_period_days: self.trial_period_days,
        })
    }
}
