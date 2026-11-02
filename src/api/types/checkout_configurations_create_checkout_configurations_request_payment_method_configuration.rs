pub use crate::prelude::*;

/// Payment method overrides for this checkout. `null` uses the plan or platform defaults.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCheckoutConfigurationsRequestPaymentMethodConfiguration {
    /// Payment method types explicitly disabled for checkout — the `type` values from the payment method types catalogue. Types Whop no longer offers, and the read-only `unknown` placeholder, are dropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<Vec<PaymentMethodTypes>>,
    /// Payment method types explicitly enabled for checkout — the `type` values from the payment method types catalogue. Types Whop no longer offers, and the read-only `unknown` placeholder, are dropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Vec<PaymentMethodTypes>>,
    /// Whether platform default payment methods are included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_platform_defaults: Option<bool>,
}

impl CreateCheckoutConfigurationsRequestPaymentMethodConfiguration {
    pub fn builder() -> CreateCheckoutConfigurationsRequestPaymentMethodConfigurationBuilder {
        <CreateCheckoutConfigurationsRequestPaymentMethodConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCheckoutConfigurationsRequestPaymentMethodConfigurationBuilder {
    disabled: Option<Vec<PaymentMethodTypes>>,
    enabled: Option<Vec<PaymentMethodTypes>>,
    include_platform_defaults: Option<bool>,
}

impl CreateCheckoutConfigurationsRequestPaymentMethodConfigurationBuilder {
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

    /// Consumes the builder and constructs a [`CreateCheckoutConfigurationsRequestPaymentMethodConfiguration`].
    pub fn build(
        self,
    ) -> Result<CreateCheckoutConfigurationsRequestPaymentMethodConfiguration, BuildError> {
        Ok(
            CreateCheckoutConfigurationsRequestPaymentMethodConfiguration {
                disabled: self.disabled,
                enabled: self.enabled,
                include_platform_defaults: self.include_platform_defaults,
            },
        )
    }
}
