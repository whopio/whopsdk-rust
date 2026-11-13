pub use crate::prelude::*;

/// Variant attributes used to create or find a variant for this checkout configuration. Mutually exclusive with `variant_id`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateCheckoutConfigurationsRequestVariant {
    /// Account ID for the inline variant, prefixed `biz_`. Defaults to the account resolved from the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Application fee the platform collects per unit, in the variant currency (5.00 means $5.00 for USD). Must be positive and below the initial price for one-time variants or renewal price for recurring variants. Paid to the parent account alongside other processing fees; collection is capped to remaining proceeds. Applies to subsequent payments on recurring variants. Only valid for subaccounts with a parent account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<f64>,
    /// Recurring billing interval in days, such as 30 for monthly or 365 for annual.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<i64>,
    /// Billing model for the variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_type: Option<CreateCheckoutConfigurationsRequestVariantBillingType>,
    /// Three-letter ISO currency code for the variant's prices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Customer-visible variant description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Access duration in days for expiration-based variants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_days: Option<i64>,
    /// Whether to create a new variant instead of reusing a matching one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_create_new_plan: Option<bool>,
    /// Initial purchase price in the variant currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_price: Option<f64>,
    /// Custom key-value metadata stored on the variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Tax classification override for this variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_tax_type: Option<String>,
    /// Payment method overrides for the inline variant. `null` uses platform defaults.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration:
        Option<CreateCheckoutConfigurationsRequestVariantPaymentMethodConfiguration>,
    /// Product ID the inline variant should belong to, prefixed `prod_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Sales method for the variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_method: Option<CreateCheckoutConfigurationsRequestVariantReleaseMethod>,
    /// Recurring price charged each billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_price: Option<f64>,
    /// Units available for purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock: Option<i64>,
    /// 3D Secure behavior for supported on-session card payments. `mandate_challenge` requires a 3DS challenge before payment processing; `mandate_if_required` mandates a challenge only when the payment processor requires it; `frictionless_if_required` uses the regular frictionless 3DS flow. Payments of $1,000 or more use `mandate_if_required` unless `mandate_challenge` is selected. Risk and authentication recovery requirements can override the preference. `null` inherits the account default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_level: Option<CreateCheckoutConfigurationsRequestVariantThreeDsLevel>,
    /// Variant display name shown to customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Free trial days before the first renewal charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    /// Whether the variant has unlimited stock.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlimited_stock: Option<bool>,
    /// Whether the variant is visible to customers or hidden from public view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<CreateCheckoutConfigurationsRequestVariantVisibility>,
}

impl CreateCheckoutConfigurationsRequestVariant {
    pub fn builder() -> CreateCheckoutConfigurationsRequestVariantBuilder {
        <CreateCheckoutConfigurationsRequestVariantBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCheckoutConfigurationsRequestVariantBuilder {
    account_id: Option<String>,
    application_fee_amount: Option<f64>,
    billing_period: Option<i64>,
    billing_type: Option<CreateCheckoutConfigurationsRequestVariantBillingType>,
    currency: Option<String>,
    description: Option<String>,
    expiration_days: Option<i64>,
    force_create_new_plan: Option<bool>,
    initial_price: Option<f64>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    override_tax_type: Option<String>,
    payment_method_configuration:
        Option<CreateCheckoutConfigurationsRequestVariantPaymentMethodConfiguration>,
    product_id: Option<String>,
    release_method: Option<CreateCheckoutConfigurationsRequestVariantReleaseMethod>,
    renewal_price: Option<f64>,
    stock: Option<i64>,
    three_ds_level: Option<CreateCheckoutConfigurationsRequestVariantThreeDsLevel>,
    title: Option<String>,
    trial_period_days: Option<i64>,
    unlimited_stock: Option<bool>,
    visibility: Option<CreateCheckoutConfigurationsRequestVariantVisibility>,
}

impl CreateCheckoutConfigurationsRequestVariantBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn application_fee_amount(mut self, value: f64) -> Self {
        self.application_fee_amount = Some(value);
        self
    }

    pub fn billing_period(mut self, value: i64) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn billing_type(
        mut self,
        value: CreateCheckoutConfigurationsRequestVariantBillingType,
    ) -> Self {
        self.billing_type = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
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
        value: CreateCheckoutConfigurationsRequestVariantPaymentMethodConfiguration,
    ) -> Self {
        self.payment_method_configuration = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn release_method(
        mut self,
        value: CreateCheckoutConfigurationsRequestVariantReleaseMethod,
    ) -> Self {
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

    pub fn three_ds_level(
        mut self,
        value: CreateCheckoutConfigurationsRequestVariantThreeDsLevel,
    ) -> Self {
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

    pub fn visibility(
        mut self,
        value: CreateCheckoutConfigurationsRequestVariantVisibility,
    ) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCheckoutConfigurationsRequestVariant`].
    pub fn build(self) -> Result<CreateCheckoutConfigurationsRequestVariant, BuildError> {
        Ok(CreateCheckoutConfigurationsRequestVariant {
            account_id: self.account_id,
            application_fee_amount: self.application_fee_amount,
            billing_period: self.billing_period,
            billing_type: self.billing_type,
            currency: self.currency,
            description: self.description,
            expiration_days: self.expiration_days,
            force_create_new_plan: self.force_create_new_plan,
            initial_price: self.initial_price,
            metadata: self.metadata,
            override_tax_type: self.override_tax_type,
            payment_method_configuration: self.payment_method_configuration,
            product_id: self.product_id,
            release_method: self.release_method,
            renewal_price: self.renewal_price,
            stock: self.stock,
            three_ds_level: self.three_ds_level,
            title: self.title,
            trial_period_days: self.trial_period_days,
            unlimited_stock: self.unlimited_stock,
            visibility: self.visibility,
        })
    }
}
