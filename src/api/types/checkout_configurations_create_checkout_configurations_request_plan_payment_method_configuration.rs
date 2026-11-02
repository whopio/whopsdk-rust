pub use crate::prelude::*;

/// Payment method overrides for the inline plan. `null` uses platform defaults.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCheckoutConfigurationsRequestPlanPaymentMethodConfiguration {
    /// Payment method types explicitly disabled for this plan — the `type` values from the payment method types catalogue. Types Whop no longer offers, and the read-only `unknown` placeholder, are dropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<Vec<PaymentMethodTypes>>,
    /// Payment method types explicitly enabled for this plan — the `type` values from the payment method types catalogue. Types Whop no longer offers, and the read-only `unknown` placeholder, are dropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Vec<PaymentMethodTypes>>,
    /// Whether platform default payment methods are included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_platform_defaults: Option<bool>,
}

impl CreateCheckoutConfigurationsRequestPlanPaymentMethodConfiguration {
    pub fn builder() -> CreateCheckoutConfigurationsRequestPlanPaymentMethodConfigurationBuilder {
        <CreateCheckoutConfigurationsRequestPlanPaymentMethodConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCheckoutConfigurationsRequestPlanPaymentMethodConfigurationBuilder {
    disabled: Option<Vec<PaymentMethodTypes>>,
    enabled: Option<Vec<PaymentMethodTypes>>,
    include_platform_defaults: Option<bool>,
}

impl CreateCheckoutConfigurationsRequestPlanPaymentMethodConfigurationBuilder {
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

    /// Consumes the builder and constructs a [`CreateCheckoutConfigurationsRequestPlanPaymentMethodConfiguration`].
    pub fn build(
        self,
    ) -> Result<CreateCheckoutConfigurationsRequestPlanPaymentMethodConfiguration, BuildError> {
        Ok(
            CreateCheckoutConfigurationsRequestPlanPaymentMethodConfiguration {
                disabled: self.disabled,
                enabled: self.enabled,
                include_platform_defaults: self.include_platform_defaults,
            },
        )
    }
}
