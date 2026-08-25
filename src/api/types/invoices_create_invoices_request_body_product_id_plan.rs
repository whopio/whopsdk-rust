pub use crate::prelude::*;

/// The plan attributes defining the price, currency, and billing interval for this invoice.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateInvoicesRequestBodyProductIdPlan {
    /// Whether this plan accepts local currency payments via adaptive pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_pricing_enabled: Option<bool>,
    /// The interval in days at which the plan charges (renewal plans).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<i64>,
    /// The three-letter ISO currency code for the plan's pricing. Defaults to USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currencies>,
    /// An array of custom field objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CreateInvoicesRequestBodyProductIdPlanCustomFieldsItem>>,
    /// The description of the plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The number of days until the membership expires and revokes access (expiration plans). For example, 365 for a one-year access period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_days: Option<i64>,
    /// An additional amount charged upon first purchase. Use only if a one time payment OR you want to charge an additional amount on top of the renewal price. Provided as a number in the specified currency. Eg: 10.43 for $10.43
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_price: Option<f64>,
    /// A personal description or notes section for the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_notes: Option<String>,
    /// Whether this plan uses legacy payment method controls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_payment_method_controls: Option<bool>,
    /// The explicit payment method configuration for the plan. If not provided, the platform or company's defaults will apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration:
        Option<CreateInvoicesRequestBodyProductIdPlanPaymentMethodConfiguration>,
    /// Indicates if the plan is a one time payment or recurring.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<PlanTypes>,
    /// This is the release method the business uses to sell this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_method: Option<ReleaseMethod>,
    /// The amount the customer is charged every billing period. Use only if a recurring payment. Provided as a number in the specified currency. Eg: 10.43 for $10.43
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_price: Option<f64>,
    /// The number of units available for purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock: Option<i64>,
    /// The number of free trial days added before a renewal plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    /// When true, the plan has unlimited stock (stock field is ignored). When false, purchases are limited by the stock field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlimited_stock: Option<bool>,
    /// Shows or hides the plan from public/business view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

impl CreateInvoicesRequestBodyProductIdPlan {
    pub fn builder() -> CreateInvoicesRequestBodyProductIdPlanBuilder {
        <CreateInvoicesRequestBodyProductIdPlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateInvoicesRequestBodyProductIdPlanBuilder {
    adaptive_pricing_enabled: Option<bool>,
    billing_period: Option<i64>,
    currency: Option<Currencies>,
    custom_fields: Option<Vec<CreateInvoicesRequestBodyProductIdPlanCustomFieldsItem>>,
    description: Option<String>,
    expiration_days: Option<i64>,
    initial_price: Option<f64>,
    internal_notes: Option<String>,
    legacy_payment_method_controls: Option<bool>,
    payment_method_configuration:
        Option<CreateInvoicesRequestBodyProductIdPlanPaymentMethodConfiguration>,
    plan_type: Option<PlanTypes>,
    release_method: Option<ReleaseMethod>,
    renewal_price: Option<f64>,
    stock: Option<i64>,
    trial_period_days: Option<i64>,
    unlimited_stock: Option<bool>,
    visibility: Option<Visibility>,
}

impl CreateInvoicesRequestBodyProductIdPlanBuilder {
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

    pub fn custom_fields(
        mut self,
        value: Vec<CreateInvoicesRequestBodyProductIdPlanCustomFieldsItem>,
    ) -> Self {
        self.custom_fields = Some(value);
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

    pub fn initial_price(mut self, value: f64) -> Self {
        self.initial_price = Some(value);
        self
    }

    pub fn internal_notes(mut self, value: impl Into<String>) -> Self {
        self.internal_notes = Some(value.into());
        self
    }

    pub fn legacy_payment_method_controls(mut self, value: bool) -> Self {
        self.legacy_payment_method_controls = Some(value);
        self
    }

    pub fn payment_method_configuration(
        mut self,
        value: CreateInvoicesRequestBodyProductIdPlanPaymentMethodConfiguration,
    ) -> Self {
        self.payment_method_configuration = Some(value);
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

    pub fn stock(mut self, value: i64) -> Self {
        self.stock = Some(value);
        self
    }

    pub fn trial_period_days(mut self, value: i64) -> Self {
        self.trial_period_days = Some(value);
        self
    }

    pub fn unlimited_stock(mut self, value: bool) -> Self {
        self.unlimited_stock = Some(value);
        self
    }

    pub fn visibility(mut self, value: Visibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateInvoicesRequestBodyProductIdPlan`].
    pub fn build(self) -> Result<CreateInvoicesRequestBodyProductIdPlan, BuildError> {
        Ok(CreateInvoicesRequestBodyProductIdPlan {
            adaptive_pricing_enabled: self.adaptive_pricing_enabled,
            billing_period: self.billing_period,
            currency: self.currency,
            custom_fields: self.custom_fields,
            description: self.description,
            expiration_days: self.expiration_days,
            initial_price: self.initial_price,
            internal_notes: self.internal_notes,
            legacy_payment_method_controls: self.legacy_payment_method_controls,
            payment_method_configuration: self.payment_method_configuration,
            plan_type: self.plan_type,
            release_method: self.release_method,
            renewal_price: self.renewal_price,
            stock: self.stock,
            trial_period_days: self.trial_period_days,
            unlimited_stock: self.unlimited_stock,
            visibility: self.visibility,
        })
    }
}
