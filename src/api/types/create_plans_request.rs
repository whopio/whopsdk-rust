pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePlansRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_pricing_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_multiple_quantity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_discount_intervals: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_discount_percentage: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_styling: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CreatePlansRequestCustomFieldsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<CreatePlansRequestImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_cancel_discount: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_tax_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration: Option<CreatePlansRequestPaymentMethodConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_pay_required_payments: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_level: Option<CreatePlansRequestThreeDsLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlimited_stock: Option<bool>,
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
    allow_multiple_quantity: Option<bool>,
    attributes: Option<HashMap<String, serde_json::Value>>,
    billing_period: Option<i64>,
    billing_type: Option<String>,
    cancel_discount_intervals: Option<i64>,
    cancel_discount_percentage: Option<i64>,
    checkout_styling: Option<HashMap<String, serde_json::Value>>,
    currency: Option<String>,
    custom_fields: Option<Vec<CreatePlansRequestCustomFieldsItem>>,
    description: Option<String>,
    expiration_days: Option<i64>,
    image: Option<CreatePlansRequestImage>,
    initial_price: Option<f64>,
    internal_notes: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    offer_cancel_discount: Option<bool>,
    override_tax_type: Option<String>,
    payment_method_configuration: Option<CreatePlansRequestPaymentMethodConfiguration>,
    product_id: Option<String>,
    release_method: Option<String>,
    renewal_price: Option<f64>,
    sku: Option<String>,
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

    pub fn allow_multiple_quantity(mut self, value: bool) -> Self {
        self.allow_multiple_quantity = Some(value);
        self
    }

    pub fn attributes(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn billing_period(mut self, value: i64) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn billing_type(mut self, value: impl Into<String>) -> Self {
        self.billing_type = Some(value.into());
        self
    }

    pub fn cancel_discount_intervals(mut self, value: i64) -> Self {
        self.cancel_discount_intervals = Some(value);
        self
    }

    pub fn cancel_discount_percentage(mut self, value: i64) -> Self {
        self.cancel_discount_percentage = Some(value);
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

    pub fn offer_cancel_discount(mut self, value: bool) -> Self {
        self.offer_cancel_discount = Some(value);
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

    pub fn sku(mut self, value: impl Into<String>) -> Self {
        self.sku = Some(value.into());
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
            allow_multiple_quantity: self.allow_multiple_quantity,
            attributes: self.attributes,
            billing_period: self.billing_period,
            billing_type: self.billing_type,
            cancel_discount_intervals: self.cancel_discount_intervals,
            cancel_discount_percentage: self.cancel_discount_percentage,
            checkout_styling: self.checkout_styling,
            currency: self.currency,
            custom_fields: self.custom_fields,
            description: self.description,
            expiration_days: self.expiration_days,
            image: self.image,
            initial_price: self.initial_price,
            internal_notes: self.internal_notes,
            metadata: self.metadata,
            offer_cancel_discount: self.offer_cancel_discount,
            override_tax_type: self.override_tax_type,
            payment_method_configuration: self.payment_method_configuration,
            product_id: self.product_id,
            release_method: self.release_method,
            renewal_price: self.renewal_price,
            sku: self.sku,
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
