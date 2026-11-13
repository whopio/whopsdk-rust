pub use crate::prelude::*;

/// Payment method overrides for the inline variant. `null` uses platform defaults.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCheckoutConfigurationsRequestVariantPaymentMethodConfiguration {
    /// Payment method types explicitly disabled for this variant — the `type` values from the payment method types catalogue. Types Whop no longer offers, and the read-only `unknown` placeholder, are dropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<Vec<PaymentMethodTypes>>,
    /// Payment method types explicitly enabled for this variant — the `type` values from the payment method types catalogue. Types Whop no longer offers, and the read-only `unknown` placeholder, are dropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Vec<PaymentMethodTypes>>,
    /// Whether platform default payment methods are included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_platform_defaults: Option<bool>,
}

impl CreateCheckoutConfigurationsRequestVariantPaymentMethodConfiguration {
    pub fn builder() -> CreateCheckoutConfigurationsRequestVariantPaymentMethodConfigurationBuilder
    {
        <CreateCheckoutConfigurationsRequestVariantPaymentMethodConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCheckoutConfigurationsRequestVariantPaymentMethodConfigurationBuilder {
    disabled: Option<Vec<PaymentMethodTypes>>,
    enabled: Option<Vec<PaymentMethodTypes>>,
    include_platform_defaults: Option<bool>,
}

impl CreateCheckoutConfigurationsRequestVariantPaymentMethodConfigurationBuilder {
    pub fn disabled(mut self, value: Vec<PaymentMethodTypes>) -> Self {
        self.disabled = Some(value);
        self
    }

    pub fn enabled(mut self, value: Vec<PaymentMethodTypes>) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn include_platform_defaults(mut self, value: bool) -> Self {
        self.include_platform_defaults = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCheckoutConfigurationsRequestVariantPaymentMethodConfiguration`].
    pub fn build(
        self,
    ) -> Result<CreateCheckoutConfigurationsRequestVariantPaymentMethodConfiguration, BuildError>
    {
        Ok(
            CreateCheckoutConfigurationsRequestVariantPaymentMethodConfiguration {
                disabled: self.disabled,
                enabled: self.enabled,
                include_platform_defaults: self.include_platform_defaults,
            },
        )
    }
}
