pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CheckoutConfigurationVariant {
    /// Whether this variant accepts local currency payments via adaptive pricing.
    #[serde(default)]
    pub adaptive_pricing_enabled: bool,
    /// Recurring billing interval in days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<i64>,
    /// Billing model for the variant.
    pub billing_type: CheckoutConfigurationVariantBillingType,
    /// Three-letter ISO currency code for the variant's prices.
    #[serde(default)]
    pub currency: String,
    /// Access duration in days for expiration-based variants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_days: Option<i64>,
    /// Variant ID, prefixed `plan_`.
    #[serde(default)]
    pub id: String,
    /// Initial purchase price in the variant currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub initial_price: f64,
    /// Sales method for the variant.
    pub release_method: CheckoutConfigurationVariantReleaseMethod,
    /// Recurring price charged each billing period.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub renewal_price: f64,
    /// 3D Secure behavior for supported on-session card payments. `mandate_challenge` requires a 3DS challenge before payment processing; `mandate_if_required` mandates a challenge only when the payment processor requires it; `frictionless_if_required` uses the regular frictionless 3DS flow. Payments of $1,000 or more use `mandate_if_required` unless `mandate_challenge` is selected. Risk and authentication recovery requirements can override the preference. `null` inherits the account default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_level: Option<CheckoutConfigurationVariantThreeDsLevel>,
    /// Free trial days before the first renewal charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    /// Whether the variant is visible to customers or hidden from public view.
    pub visibility: CheckoutConfigurationVariantVisibility,
}

impl CheckoutConfigurationVariant {
    pub fn builder() -> CheckoutConfigurationVariantBuilder {
        <CheckoutConfigurationVariantBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutConfigurationVariantBuilder {
    adaptive_pricing_enabled: Option<bool>,
    billing_period: Option<i64>,
    billing_type: Option<CheckoutConfigurationVariantBillingType>,
    currency: Option<String>,
    expiration_days: Option<i64>,
    id: Option<String>,
    initial_price: Option<f64>,
    release_method: Option<CheckoutConfigurationVariantReleaseMethod>,
    renewal_price: Option<f64>,
    three_ds_level: Option<CheckoutConfigurationVariantThreeDsLevel>,
    trial_period_days: Option<i64>,
    visibility: Option<CheckoutConfigurationVariantVisibility>,
}

impl CheckoutConfigurationVariantBuilder {
    pub fn adaptive_pricing_enabled(mut self, value: bool) -> Self {
        self.adaptive_pricing_enabled = Some(value);
        self
    }

    pub fn billing_period(mut self, value: i64) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn billing_type(mut self, value: CheckoutConfigurationVariantBillingType) -> Self {
        self.billing_type = Some(value);
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

    pub fn release_method(mut self, value: CheckoutConfigurationVariantReleaseMethod) -> Self {
        self.release_method = Some(value);
        self
    }

    pub fn renewal_price(mut self, value: f64) -> Self {
        self.renewal_price = Some(value);
        self
    }

    pub fn three_ds_level(mut self, value: CheckoutConfigurationVariantThreeDsLevel) -> Self {
        self.three_ds_level = Some(value);
        self
    }

    pub fn trial_period_days(mut self, value: i64) -> Self {
        self.trial_period_days = Some(value);
        self
    }

    pub fn visibility(mut self, value: CheckoutConfigurationVariantVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckoutConfigurationVariant`].
    /// This method will fail if any of the following fields are not set:
    /// - [`adaptive_pricing_enabled`](CheckoutConfigurationVariantBuilder::adaptive_pricing_enabled)
    /// - [`billing_type`](CheckoutConfigurationVariantBuilder::billing_type)
    /// - [`currency`](CheckoutConfigurationVariantBuilder::currency)
    /// - [`id`](CheckoutConfigurationVariantBuilder::id)
    /// - [`initial_price`](CheckoutConfigurationVariantBuilder::initial_price)
    /// - [`release_method`](CheckoutConfigurationVariantBuilder::release_method)
    /// - [`renewal_price`](CheckoutConfigurationVariantBuilder::renewal_price)
    /// - [`visibility`](CheckoutConfigurationVariantBuilder::visibility)
    pub fn build(self) -> Result<CheckoutConfigurationVariant, BuildError> {
        Ok(CheckoutConfigurationVariant {
            adaptive_pricing_enabled: self
                .adaptive_pricing_enabled
                .ok_or_else(|| BuildError::missing_field("adaptive_pricing_enabled"))?,
            billing_period: self.billing_period,
            billing_type: self
                .billing_type
                .ok_or_else(|| BuildError::missing_field("billing_type"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            expiration_days: self.expiration_days,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            initial_price: self
                .initial_price
                .ok_or_else(|| BuildError::missing_field("initial_price"))?,
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
