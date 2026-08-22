pub use crate::prelude::*;

/// Pass this object to create a new plan for this payment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatePaymentsRequestOnePlan {
    /// The application fee amount collected by the platform from this connected account. Provided as a number in dollars (e.g., 5.00 for $5.00). Must be less than the total payment amount. Only valid for connected accounts with a parent company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<f64>,
    /// The interval in days at which the plan charges (renewal plans). For example, 30 for monthly billing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<i64>,
    /// The respective currency identifier for the plan.
    pub currency: Currencies,
    /// The description of the plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The number of days until the membership expires and revokes access (expiration plans). For example, 365 for one year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_days: Option<i64>,
    /// Whether to force the creation of a new plan even if one with the same attributes already exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_create_new_plan: Option<bool>,
    /// An additional amount charged upon first purchase. Provided as a number in the specified currency. Eg: 10.43 for $10.43 USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_price: Option<f64>,
    /// A personal description or notes section for the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_notes: Option<String>,
    /// Indicates if the plan is a one time payment or recurring.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<PlanTypes>,
    /// Pass this object to create a new product for this plan. We will use the product external identifier to find or create an existing product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<CreatePaymentsRequestOnePlanProduct>,
    /// The product the plan is related to. Either this or product is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// The amount the customer is charged every billing period. Provided as a number in the specified currency. Eg: 10.43 for $10.43 USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_price: Option<f64>,
    /// The title of the plan. This will be visible on the product page to customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The number of free trial days added before a renewal plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    /// Shows or hides the plan from public/business view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

impl CreatePaymentsRequestOnePlan {
    pub fn builder() -> CreatePaymentsRequestOnePlanBuilder {
        <CreatePaymentsRequestOnePlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentsRequestOnePlanBuilder {
    application_fee_amount: Option<f64>,
    billing_period: Option<i64>,
    currency: Option<Currencies>,
    description: Option<String>,
    expiration_days: Option<i64>,
    force_create_new_plan: Option<bool>,
    initial_price: Option<f64>,
    internal_notes: Option<String>,
    plan_type: Option<PlanTypes>,
    product: Option<CreatePaymentsRequestOnePlanProduct>,
    product_id: Option<String>,
    renewal_price: Option<f64>,
    title: Option<String>,
    trial_period_days: Option<i64>,
    visibility: Option<Visibility>,
}

impl CreatePaymentsRequestOnePlanBuilder {
    pub fn application_fee_amount(mut self, value: f64) -> Self {
        self.application_fee_amount = Some(value);
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

    pub fn plan_type(mut self, value: PlanTypes) -> Self {
        self.plan_type = Some(value);
        self
    }

    pub fn product(mut self, value: CreatePaymentsRequestOnePlanProduct) -> Self {
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

    pub fn visibility(mut self, value: Visibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentsRequestOnePlan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](CreatePaymentsRequestOnePlanBuilder::currency)
    pub fn build(self) -> Result<CreatePaymentsRequestOnePlan, BuildError> {
        Ok(CreatePaymentsRequestOnePlan {
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
