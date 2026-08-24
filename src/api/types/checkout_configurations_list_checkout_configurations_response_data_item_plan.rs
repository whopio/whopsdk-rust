pub use crate::prelude::*;

/// Plan used for payment checkout. `null` in setup mode.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCheckoutConfigurationsResponseDataItemPlan {
    /// Whether this plan accepts local currency payments via adaptive pricing.
    #[serde(default)]
    pub adaptive_pricing_enabled: bool,
    /// Recurring billing interval in days, such as 30 for monthly or 365 for annual. `null` for one-time plans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<i64>,
    /// Three-letter ISO currency code for the plan's prices.
    #[serde(default)]
    pub currency: String,
    /// Access duration in days for expiration-based plans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_days: Option<i64>,
    /// Plan ID, prefixed `plan_`.
    #[serde(default)]
    pub id: String,
    /// Initial purchase price in the plan currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub initial_price: f64,
    /// Billing model for the plan.
    pub plan_type: ListCheckoutConfigurationsResponseDataItemPlanPlanType,
    /// Sales method for the plan.
    pub release_method: ListCheckoutConfigurationsResponseDataItemPlanReleaseMethod,
    /// Recurring price charged each billing period.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub renewal_price: f64,
    /// 3D Secure behavior for this plan, or `null` to use the account default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_level: Option<ListCheckoutConfigurationsResponseDataItemPlanThreeDsLevel>,
    /// Free trial days before the first renewal charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    /// Whether the plan is visible to customers or hidden from public view.
    pub visibility: ListCheckoutConfigurationsResponseDataItemPlanVisibility,
}

impl ListCheckoutConfigurationsResponseDataItemPlan {
    pub fn builder() -> ListCheckoutConfigurationsResponseDataItemPlanBuilder {
        <ListCheckoutConfigurationsResponseDataItemPlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCheckoutConfigurationsResponseDataItemPlanBuilder {
    adaptive_pricing_enabled: Option<bool>,
    billing_period: Option<i64>,
    currency: Option<String>,
    expiration_days: Option<i64>,
    id: Option<String>,
    initial_price: Option<f64>,
    plan_type: Option<ListCheckoutConfigurationsResponseDataItemPlanPlanType>,
    release_method: Option<ListCheckoutConfigurationsResponseDataItemPlanReleaseMethod>,
    renewal_price: Option<f64>,
    three_ds_level: Option<ListCheckoutConfigurationsResponseDataItemPlanThreeDsLevel>,
    trial_period_days: Option<i64>,
    visibility: Option<ListCheckoutConfigurationsResponseDataItemPlanVisibility>,
}

impl ListCheckoutConfigurationsResponseDataItemPlanBuilder {
    pub fn adaptive_pricing_enabled(mut self, value: bool) -> Self {
        self.adaptive_pricing_enabled = Some(value);
        self
    }

    pub fn billing_period(mut self, value: i64) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
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

    pub fn plan_type(
        mut self,
        value: ListCheckoutConfigurationsResponseDataItemPlanPlanType,
    ) -> Self {
        self.plan_type = Some(value);
        self
    }

    pub fn release_method(
        mut self,
        value: ListCheckoutConfigurationsResponseDataItemPlanReleaseMethod,
    ) -> Self {
        self.release_method = Some(value);
        self
    }

    pub fn renewal_price(mut self, value: f64) -> Self {
        self.renewal_price = Some(value);
        self
    }

    pub fn three_ds_level(
        mut self,
        value: ListCheckoutConfigurationsResponseDataItemPlanThreeDsLevel,
    ) -> Self {
        self.three_ds_level = Some(value);
        self
    }

    pub fn trial_period_days(mut self, value: i64) -> Self {
        self.trial_period_days = Some(value);
        self
    }

    pub fn visibility(
        mut self,
        value: ListCheckoutConfigurationsResponseDataItemPlanVisibility,
    ) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCheckoutConfigurationsResponseDataItemPlan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`adaptive_pricing_enabled`](ListCheckoutConfigurationsResponseDataItemPlanBuilder::adaptive_pricing_enabled)
    /// - [`currency`](ListCheckoutConfigurationsResponseDataItemPlanBuilder::currency)
    /// - [`id`](ListCheckoutConfigurationsResponseDataItemPlanBuilder::id)
    /// - [`initial_price`](ListCheckoutConfigurationsResponseDataItemPlanBuilder::initial_price)
    /// - [`plan_type`](ListCheckoutConfigurationsResponseDataItemPlanBuilder::plan_type)
    /// - [`release_method`](ListCheckoutConfigurationsResponseDataItemPlanBuilder::release_method)
    /// - [`renewal_price`](ListCheckoutConfigurationsResponseDataItemPlanBuilder::renewal_price)
    /// - [`visibility`](ListCheckoutConfigurationsResponseDataItemPlanBuilder::visibility)
    pub fn build(self) -> Result<ListCheckoutConfigurationsResponseDataItemPlan, BuildError> {
        Ok(ListCheckoutConfigurationsResponseDataItemPlan {
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
