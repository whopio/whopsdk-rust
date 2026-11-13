pub use crate::prelude::*;

/// The plan that this invoice charges for.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InvoiceLegacyCurrentPlan {
    /// Number of days between recurring charges, such as 30 for monthly or 365 for annual. `null` for one-time plans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<i64>,
    /// The currency used for all prices on this plan (e.g., 'usd', 'eur'). All monetary amounts on the plan are denominated in this currency.
    pub currency: Currencies,
    /// A text description of the plan visible to customers. Maximum 1000 characters. Null if no description is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The formatted price (including currency) for the plan.
    #[serde(default)]
    pub formatted_price: String,
    /// The unique identifier for the plan.
    #[serde(default)]
    pub id: String,
    /// The initial purchase price in the plan's base_currency (e.g., 49.99 for $49.99). For one-time plans, this is the full price. For renewal plans, this is charged on top of the first renewal_price.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub initial_price: f64,
    /// The billing model for this plan: 'renewal' for recurring subscriptions or 'one_time' for single payments.
    pub plan_type: PlanTypes,
    /// The recurring price charged every billing_period in the plan's base_currency (e.g., 9.99 for $9.99/period). Zero for one-time plans.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub renewal_price: f64,
    /// Free trial days before first renewal charge. `null` if no trial is configured or the user has already used a trial for this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
}

impl InvoiceLegacyCurrentPlan {
    pub fn builder() -> InvoiceLegacyCurrentPlanBuilder {
        <InvoiceLegacyCurrentPlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceLegacyCurrentPlanBuilder {
    billing_period: Option<i64>,
    currency: Option<Currencies>,
    description: Option<String>,
    formatted_price: Option<String>,
    id: Option<String>,
    initial_price: Option<f64>,
    plan_type: Option<PlanTypes>,
    renewal_price: Option<f64>,
    trial_period_days: Option<i64>,
}

impl InvoiceLegacyCurrentPlanBuilder {
    pub fn billing_period(mut self, value: i64) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn currency(mut self, value: Currencies) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn formatted_price(mut self, value: impl Into<String>) -> Self {
        self.formatted_price = Some(value.into());
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

    pub fn renewal_price(mut self, value: f64) -> Self {
        self.renewal_price = Some(value);
        self
    }

    pub fn trial_period_days(mut self, value: i64) -> Self {
        self.trial_period_days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoiceLegacyCurrentPlan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](InvoiceLegacyCurrentPlanBuilder::currency)
    /// - [`formatted_price`](InvoiceLegacyCurrentPlanBuilder::formatted_price)
    /// - [`id`](InvoiceLegacyCurrentPlanBuilder::id)
    /// - [`initial_price`](InvoiceLegacyCurrentPlanBuilder::initial_price)
    /// - [`plan_type`](InvoiceLegacyCurrentPlanBuilder::plan_type)
    /// - [`renewal_price`](InvoiceLegacyCurrentPlanBuilder::renewal_price)
    pub fn build(self) -> Result<InvoiceLegacyCurrentPlan, BuildError> {
        Ok(InvoiceLegacyCurrentPlan {
            billing_period: self.billing_period,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            description: self.description,
            formatted_price: self
                .formatted_price
                .ok_or_else(|| BuildError::missing_field("formatted_price"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            initial_price: self
                .initial_price
                .ok_or_else(|| BuildError::missing_field("initial_price"))?,
            plan_type: self
                .plan_type
                .ok_or_else(|| BuildError::missing_field("plan_type"))?,
            renewal_price: self
                .renewal_price
                .ok_or_else(|| BuildError::missing_field("renewal_price"))?,
            trial_period_days: self.trial_period_days,
        })
    }
}
