pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePlansRequest {
    /// The unique identifier of the account to create this plan for. Defaults to the caller's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Whether this plan accepts local currency payments via adaptive pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_pricing_enabled: Option<bool>,
    /// Recurring billing interval in days, such as 30 for monthly or 365 for annual.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<i64>,
    /// Checkout styling overrides for this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_styling: Option<HashMap<String, serde_json::Value>>,
    /// The three-letter ISO currency code for the plan's pricing. Defaults to USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// An array of custom field definitions to collect from customers at checkout. Omitting this field clears existing custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CreatePlansRequestCustomFieldsItem>>,
    /// A text description of the plan displayed to customers on the product page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Access duration in days before the membership expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_days: Option<i64>,
    /// An image displayed on the product page to represent this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<CreatePlansRequestImage>,
    /// Initial amount charged in the plan's currency, e.g. 10.43 for $10.43.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_price: Option<f64>,
    /// Private notes visible only to the account owner. Not shown to customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_notes: Option<String>,
    /// Custom key-value pairs to store on the plan. Included in webhook payloads for payment and membership events. Max 50 keys, 100 chars per key, 500 chars per string value. The reserved keys `custom_cta` (a checkout call-to-action button label — one of the product custom CTA values, e.g. `subscribe`, `get_offer`) and `custom_cta_url` (a URL the button links to; web or `tel:`) override the product's call to action for this plan and are validated on save.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Override the default tax classification for this specific plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_tax_type: Option<String>,
    /// Explicit payment method configuration for the plan. When not provided, the account's defaults apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration: Option<CreatePlansRequestPaymentMethodConfiguration>,
    /// Plan billing type, such as `one_time` or `renewal`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    /// The unique identifier of the product to attach this plan to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Sales method for this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_method: Option<String>,
    /// The amount charged each billing period for recurring plans, in the plan's currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_price: Option<f64>,
    /// Installment payments required before the subscription pauses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_pay_required_payments: Option<i64>,
    /// The maximum number of units available for purchase. Ignored when unlimited_stock is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock: Option<i64>,
    /// 3D Secure behavior for this plan. Send `null` to inherit the account default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_level: Option<CreatePlansRequestThreeDsLevel>,
    /// The display name of the plan shown to customers on the product page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Free trial duration before the first recurring charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    /// Whether the plan has unlimited stock. When true, the stock field is ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlimited_stock: Option<bool>,
    /// Whether the plan is visible to customers or hidden from public view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

impl CreatePlansRequest {
    pub fn builder() -> CreatePlansRequestBuilder {
        <CreatePlansRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePlansRequestBuilder {
    account_id: Option<String>,
    adaptive_pricing_enabled: Option<bool>,
    billing_period: Option<i64>,
    checkout_styling: Option<HashMap<String, serde_json::Value>>,
    currency: Option<String>,
    custom_fields: Option<Vec<CreatePlansRequestCustomFieldsItem>>,
    description: Option<String>,
    expiration_days: Option<i64>,
    image: Option<CreatePlansRequestImage>,
    initial_price: Option<f64>,
    internal_notes: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    override_tax_type: Option<String>,
    payment_method_configuration: Option<CreatePlansRequestPaymentMethodConfiguration>,
    plan_type: Option<String>,
    product_id: Option<String>,
    release_method: Option<String>,
    renewal_price: Option<f64>,
    split_pay_required_payments: Option<i64>,
    stock: Option<i64>,
    three_ds_level: Option<CreatePlansRequestThreeDsLevel>,
    title: Option<String>,
    trial_period_days: Option<i64>,
    unlimited_stock: Option<bool>,
    visibility: Option<String>,
}

impl CreatePlansRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn adaptive_pricing_enabled(mut self, value: bool) -> Self {
        self.adaptive_pricing_enabled = Some(value);
        self
    }

    pub fn billing_period(mut self, value: i64) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn checkout_styling(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.checkout_styling = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn custom_fields(mut self, value: Vec<CreatePlansRequestCustomFieldsItem>) -> Self {
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

    pub fn image(mut self, value: CreatePlansRequestImage) -> Self {
        self.image = Some(value);
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

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn override_tax_type(mut self, value: impl Into<String>) -> Self {
        self.override_tax_type = Some(value.into());
        self
    }

    pub fn payment_method_configuration(
        mut self,
        value: CreatePlansRequestPaymentMethodConfiguration,
    ) -> Self {
        self.payment_method_configuration = Some(value);
        self
    }

    pub fn plan_type(mut self, value: impl Into<String>) -> Self {
        self.plan_type = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn release_method(mut self, value: impl Into<String>) -> Self {
        self.release_method = Some(value.into());
        self
    }

    pub fn renewal_price(mut self, value: f64) -> Self {
        self.renewal_price = Some(value);
        self
    }

    pub fn split_pay_required_payments(mut self, value: i64) -> Self {
        self.split_pay_required_payments = Some(value);
        self
    }

    pub fn stock(mut self, value: i64) -> Self {
        self.stock = Some(value);
        self
    }

    pub fn three_ds_level(mut self, value: CreatePlansRequestThreeDsLevel) -> Self {
        self.three_ds_level = Some(value);
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

    pub fn unlimited_stock(mut self, value: bool) -> Self {
        self.unlimited_stock = Some(value);
        self
    }

    pub fn visibility(mut self, value: impl Into<String>) -> Self {
        self.visibility = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePlansRequest`].
    pub fn build(self) -> Result<CreatePlansRequest, BuildError> {
        Ok(CreatePlansRequest {
            account_id: self.account_id,
            adaptive_pricing_enabled: self.adaptive_pricing_enabled,
            billing_period: self.billing_period,
            checkout_styling: self.checkout_styling,
            currency: self.currency,
            custom_fields: self.custom_fields,
            description: self.description,
            expiration_days: self.expiration_days,
            image: self.image,
            initial_price: self.initial_price,
            internal_notes: self.internal_notes,
            metadata: self.metadata,
            override_tax_type: self.override_tax_type,
            payment_method_configuration: self.payment_method_configuration,
            plan_type: self.plan_type,
            product_id: self.product_id,
            release_method: self.release_method,
            renewal_price: self.renewal_price,
            split_pay_required_payments: self.split_pay_required_payments,
            stock: self.stock,
            three_ds_level: self.three_ds_level,
            title: self.title,
            trial_period_days: self.trial_period_days,
            unlimited_stock: self.unlimited_stock,
            visibility: self.visibility,
        })
    }
}
