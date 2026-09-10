pub use crate::prelude::*;

/// Find or create a plan for this payment. Mutually exclusive with `plan_id`. Creating a plan requires plan:create; creating or updating a product requires the corresponding product permission.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatePaymentsRequestPlan {
    /// Application fee collected by the platform in the plan currency (5.00 means $5.00 for USD). Must be positive and below the initial price for one-time plans or renewal price for recurring plans. Paid to the parent account alongside other processing fees; collection is capped to remaining proceeds. Applies to subsequent payments on recurring plans. Only valid for connected accounts with a parent account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<f64>,
    /// Recurring billing interval in days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<i64>,
    /// Currency code for the plan prices.
    pub currency: CreatePaymentsRequestPlanCurrency,
    /// Plan description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Days until access expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_days: Option<i64>,
    /// Create a new plan instead of reusing a matching plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_create_new_plan: Option<bool>,
    /// Additional amount charged on the first purchase, in the plan currency. For recurring plans without a trial, the first charge includes this amount plus renewal_price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_price: Option<f64>,
    /// Internal notes for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_notes: Option<String>,
    /// Billing model for the plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<CreatePaymentsRequestPlanPlanType>,
    /// Find or create a product by external identifier. Mutually exclusive with product_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<CreatePaymentsRequestPlanProduct>,
    /// Existing product ID belonging to the account, prefixed `prod_`. Mutually exclusive with `product`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Recurring price in the plan currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_price: Option<f64>,
    /// Plan title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Free trial days before renewal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    /// Whether the plan is visible to customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<CreatePaymentsRequestPlanVisibility>,
}

impl CreatePaymentsRequestPlan {
    pub fn builder() -> CreatePaymentsRequestPlanBuilder {
        <CreatePaymentsRequestPlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentsRequestPlanBuilder {
    application_fee_amount: Option<f64>,
    billing_period: Option<i64>,
    currency: Option<CreatePaymentsRequestPlanCurrency>,
    description: Option<String>,
    expiration_days: Option<i64>,
    force_create_new_plan: Option<bool>,
    initial_price: Option<f64>,
    internal_notes: Option<String>,
    plan_type: Option<CreatePaymentsRequestPlanPlanType>,
    product: Option<CreatePaymentsRequestPlanProduct>,
    product_id: Option<String>,
    renewal_price: Option<f64>,
    title: Option<String>,
    trial_period_days: Option<i64>,
    visibility: Option<CreatePaymentsRequestPlanVisibility>,
}

impl CreatePaymentsRequestPlanBuilder {
    pub fn application_fee_amount(mut self, value: f64) -> Self {
        self.application_fee_amount = Some(value);
        self
    }

    pub fn billing_period(mut self, value: i64) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn currency(mut self, value: CreatePaymentsRequestPlanCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn expiration_days(mut self, value: i64) -> Self {
        self.expiration_days = Some(value);
        self
    }

    pub fn force_create_new_plan(mut self, value: bool) -> Self {
        self.force_create_new_plan = Some(value);
        self
    }

    pub fn initial_price(mut self, value: f64) -> Self {
        self.initial_price = Some(value);
        self
    }

    pub fn internal_notes(mut self, value: impl Into<String>) -> Self {
        self.internal_notes = Some(value.into());
        self
    }

    pub fn plan_type(mut self, value: CreatePaymentsRequestPlanPlanType) -> Self {
        self.plan_type = Some(value);
        self
    }

    pub fn product(mut self, value: CreatePaymentsRequestPlanProduct) -> Self {
        self.product = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn renewal_price(mut self, value: f64) -> Self {
        self.renewal_price = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn trial_period_days(mut self, value: i64) -> Self {
        self.trial_period_days = Some(value);
        self
    }

    pub fn visibility(mut self, value: CreatePaymentsRequestPlanVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentsRequestPlan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](CreatePaymentsRequestPlanBuilder::currency)
    pub fn build(self) -> Result<CreatePaymentsRequestPlan, BuildError> {
        Ok(CreatePaymentsRequestPlan {
            application_fee_amount: self.application_fee_amount,
            billing_period: self.billing_period,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            description: self.description,
            expiration_days: self.expiration_days,
            force_create_new_plan: self.force_create_new_plan,
            initial_price: self.initial_price,
            internal_notes: self.internal_notes,
            plan_type: self.plan_type,
            product: self.product,
            product_id: self.product_id,
            renewal_price: self.renewal_price,
            title: self.title,
            trial_period_days: self.trial_period_days,
            visibility: self.visibility,
        })
    }
}
