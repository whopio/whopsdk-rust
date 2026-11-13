pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateVariantsRequest {
    /// The unique identifier of the account to create this variant for. Required when authenticating as a user; an account API key supplies its own account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Whether this variant accepts local currency payments via adaptive pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_pricing_enabled: Option<bool>,
    /// Whether more than one unit can be purchased in an order. Renewal variants cannot enable it. When `false`, checkouts, carts, and invoices reject a quantity above 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_multiple_quantity: Option<bool>,
    /// Attribute values that make this variant one variant of its product, as a map of attribute name to value, e.g. `{"size": "Large", "color": "Blue"}`. Names are normalized to snake_case identifiers (`Ring Size` becomes `ring_size`) and come back in alphabetical order. Every variant on a product must carry the same attribute names and a distinct set of values. Send `null` to make the variant an ordinary pricing option again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, serde_json::Value>>,
    /// Recurring billing interval in days, such as 30 for monthly or 365 for annual.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<i64>,
    /// Variant billing type, such as `one_time` or `renewal`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_type: Option<String>,
    /// How long the retention discount lasts: `0` forever, `1` the next payment only, or a number of calendar months from when the customer accepts it. Required when `offer_cancel_discount` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_discount_intervals: Option<i64>,
    /// Percentage taken off each discounted renewal, as a whole number from 1 to 100. Required when `offer_cancel_discount` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_discount_percentage: Option<i64>,
    /// Checkout styling overrides for this variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_styling: Option<HashMap<String, serde_json::Value>>,
    /// The three-letter ISO currency code for the variant's pricing. Defaults to USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// An array of custom field definitions to collect from customers at checkout. Omitting this field clears existing custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CreateVariantsRequestCustomFieldsItem>>,
    /// A text description of the variant displayed to customers on the product page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Access duration in days before the membership expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_days: Option<i64>,
    /// An image displayed on the product page to represent this variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<CreateVariantsRequestImage>,
    /// Initial amount charged in the variant's currency, e.g. 10.43 for $10.43. A paid fiat variant charges at least 1.00 in its currency; use 0 for free.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_price: Option<f64>,
    /// Private notes visible only to the account owner. Not shown to customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_notes: Option<String>,
    /// Custom key-value pairs to store on the variant. Included in webhook payloads for payment and membership events. Max 50 keys, 100 chars per key, 500 chars per string value. The reserved keys `custom_cta` (a checkout call-to-action button label — one of the product custom CTA values, e.g. `subscribe`, `get_offer`) and `custom_cta_url` (a URL the button links to; web or `tel:`) override the product's call to action for this variant and are validated on save.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Whether to offer a retention discount when a customer attempts to cancel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_cancel_discount: Option<bool>,
    /// Override the default tax classification for this specific variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_tax_type: Option<String>,
    /// Explicit payment method configuration for the variant. When not provided, the account's defaults apply. Send at least one of `enabled` or `disabled`; an omitted one is empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration: Option<CreateVariantsRequestPaymentMethodConfiguration>,
    /// The unique identifier of the product to attach this variant to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Sales method for this variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_method: Option<String>,
    /// The amount charged each billing period for recurring variants, in the variant's currency. A paid fiat variant charges at least 1.00 in its currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_price: Option<f64>,
    /// Stock keeping unit for this variant. Maximum 100 characters. Free text, not enforced unique.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    /// Installment payments required before the subscription pauses, from 2 to 115. Only valid on `renewal` variants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_pay_required_payments: Option<i64>,
    /// The maximum number of units available for purchase. Ignored when unlimited_stock is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock: Option<i64>,
    /// 3D Secure behavior for supported on-session card payments. `mandate_challenge` requires a 3DS challenge before payment processing; `mandate_if_required` mandates a challenge only when the payment processor requires it; `frictionless_if_required` uses the regular frictionless 3DS flow. Payments of $1,000 or more use `mandate_if_required` unless `mandate_challenge` is selected. Risk and authentication recovery requirements can override the preference. Send `null` to inherit the account default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_level: Option<CreateVariantsRequestThreeDsLevel>,
    /// The display name of the variant shown to customers on the product page. Maximum 30 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Free trial duration before the first recurring charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    /// Whether the variant has unlimited stock. When true, the stock field is ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlimited_stock: Option<bool>,
    /// Whether the variant is visible to customers or hidden from public view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

impl CreateVariantsRequest {
    pub fn builder() -> CreateVariantsRequestBuilder {
        <CreateVariantsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateVariantsRequestBuilder {
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
    custom_fields: Option<Vec<CreateVariantsRequestCustomFieldsItem>>,
    description: Option<String>,
    expiration_days: Option<i64>,
    image: Option<CreateVariantsRequestImage>,
    initial_price: Option<f64>,
    internal_notes: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    offer_cancel_discount: Option<bool>,
    override_tax_type: Option<String>,
    payment_method_configuration: Option<CreateVariantsRequestPaymentMethodConfiguration>,
    product_id: Option<String>,
    release_method: Option<String>,
    renewal_price: Option<f64>,
    sku: Option<String>,
    split_pay_required_payments: Option<i64>,
    stock: Option<i64>,
    three_ds_level: Option<CreateVariantsRequestThreeDsLevel>,
    title: Option<String>,
    trial_period_days: Option<i64>,
    unlimited_stock: Option<bool>,
    visibility: Option<String>,
}

impl CreateVariantsRequestBuilder {
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

    pub fn custom_fields(mut self, value: Vec<CreateVariantsRequestCustomFieldsItem>) -> Self {
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

    pub fn image(mut self, value: CreateVariantsRequestImage) -> Self {
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
        value: CreateVariantsRequestPaymentMethodConfiguration,
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

    pub fn three_ds_level(mut self, value: CreateVariantsRequestThreeDsLevel) -> Self {
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

    /// Consumes the builder and constructs a [`CreateVariantsRequest`].
    pub fn build(self) -> Result<CreateVariantsRequest, BuildError> {
        Ok(CreateVariantsRequest {
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
