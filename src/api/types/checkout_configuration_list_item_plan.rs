pub use crate::prelude::*;

/// The plan to use for the checkout configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CheckoutConfigurationListItemPlan {
    /// Whether the creator has turned on adaptive pricing for this plan. Raw setting — does not check processor compatibility or feature flags.
    #[serde(default)]
    pub adaptive_pricing_enabled: bool,
    /// Number of days between recurring charges, such as 30 for monthly or 365 for annual. `null` for one-time plans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<i64>,
    /// The currency used for all prices on this plan (e.g., 'usd', 'eur'). All monetary amounts on the plan are denominated in this currency.
    pub currency: Currencies,
    /// Access duration in days for expiration-based plans, such as 365 for a one-year pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_days: Option<i64>,
    /// The unique identifier for the plan.
    #[serde(default)]
    pub id: String,
    /// The initial purchase price in the plan's base_currency (e.g., 49.99 for $49.99). For one-time plans, this is the full price. For renewal plans, this is charged on top of the first renewal_price.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub initial_price: f64,
    /// The billing model for this plan: 'renewal' for recurring subscriptions or 'one_time' for single payments.
    pub plan_type: PlanTypes,
    /// Sales method for this plan: `buy_now` for immediate purchase or `waitlist` for waitlist-based access.
    pub release_method: ReleaseMethod,
    /// The recurring price charged every billing_period in the plan's base_currency (e.g., 9.99 for $9.99/period). Zero for one-time plans.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub renewal_price: f64,
    /// The 3D Secure behavior for this plan. Null means the plan inherits the account default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_level: Option<PlanThreeDsLevels>,
    /// Free trial days before first renewal charge. `null` if no trial is configured or the user has already used a trial for this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    /// Controls whether the plan is visible to customers. When set to 'hidden', the plan is only accessible via direct link.
    pub visibility: Visibility,
}

impl CheckoutConfigurationListItemPlan {
    pub fn builder() -> CheckoutConfigurationListItemPlanBuilder {
        <CheckoutConfigurationListItemPlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutConfigurationListItemPlanBuilder {
    adaptive_pricing_enabled: Option<bool>,
    billing_period: Option<i64>,
    currency: Option<Currencies>,
    expiration_days: Option<i64>,
    id: Option<String>,
    initial_price: Option<f64>,
    plan_type: Option<PlanTypes>,
    release_method: Option<ReleaseMethod>,
    renewal_price: Option<f64>,
    three_ds_level: Option<PlanThreeDsLevels>,
    trial_period_days: Option<i64>,
    visibility: Option<Visibility>,
}

impl CheckoutConfigurationListItemPlanBuilder {
    pub fn adaptive_pricing_enabled(mut self, value: bool) -> Self {
        self.adaptive_pricing_enabled = Some(value);
        self
    }

    pub fn billing_period(mut self, value: i64) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn currency(mut self, value: Currencies) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn expiration_days(mut self, value: i64) -> Self {
        self.expiration_days = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn initial_price(mut self, value: f64) -> Self {
        self.initial_price = Some(value);
        self
    }

    pub fn plan_type(mut self, value: PlanTypes) -> Self {
        self.plan_type = Some(value);
        self
    }

    pub fn release_method(mut self, value: ReleaseMethod) -> Self {
        self.release_method = Some(value);
        self
    }

    pub fn renewal_price(mut self, value: f64) -> Self {
        self.renewal_price = Some(value);
        self
    }

    pub fn three_ds_level(mut self, value: PlanThreeDsLevels) -> Self {
        self.three_ds_level = Some(value);
        self
    }

    pub fn trial_period_days(mut self, value: i64) -> Self {
        self.trial_period_days = Some(value);
        self
    }

    pub fn visibility(mut self, value: Visibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckoutConfigurationListItemPlan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`adaptive_pricing_enabled`](CheckoutConfigurationListItemPlanBuilder::adaptive_pricing_enabled)
    /// - [`currency`](CheckoutConfigurationListItemPlanBuilder::currency)
    /// - [`id`](CheckoutConfigurationListItemPlanBuilder::id)
    /// - [`initial_price`](CheckoutConfigurationListItemPlanBuilder::initial_price)
    /// - [`plan_type`](CheckoutConfigurationListItemPlanBuilder::plan_type)
    /// - [`release_method`](CheckoutConfigurationListItemPlanBuilder::release_method)
    /// - [`renewal_price`](CheckoutConfigurationListItemPlanBuilder::renewal_price)
    /// - [`visibility`](CheckoutConfigurationListItemPlanBuilder::visibility)
    pub fn build(self) -> Result<CheckoutConfigurationListItemPlan, BuildError> {
        Ok(CheckoutConfigurationListItemPlan {
            adaptive_pricing_enabled: self
                .adaptive_pricing_enabled
                .ok_or_else(|| BuildError::missing_field("adaptive_pricing_enabled"))?,
            billing_period: self.billing_period,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            expiration_days: self.expiration_days,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            initial_price: self
                .initial_price
                .ok_or_else(|| BuildError::missing_field("initial_price"))?,
            plan_type: self
                .plan_type
                .ok_or_else(|| BuildError::missing_field("plan_type"))?,
            release_method: self
                .release_method
                .ok_or_else(|| BuildError::missing_field("release_method"))?,
            renewal_price: self
                .renewal_price
                .ok_or_else(|| BuildError::missing_field("renewal_price"))?,
            three_ds_level: self.three_ds_level,
            trial_period_days: self.trial_period_days,
            visibility: self
                .visibility
                .ok_or_else(|| BuildError::missing_field("visibility"))?,
        })
    }
}
