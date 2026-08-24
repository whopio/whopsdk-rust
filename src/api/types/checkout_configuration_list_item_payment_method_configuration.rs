pub use crate::prelude::*;

/// The explicit payment method configuration for the session, if any. This currently only works in 'setup' mode. Use the plan's payment_method_configuration for payment method.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckoutConfigurationListItemPaymentMethodConfiguration {
    /// An array of payment method identifiers that are explicitly disabled. Only applies if the include_platform_defaults is true.
    #[serde(default)]
    pub disabled: Vec<PaymentMethodTypes>,
    /// An array of payment method identifiers that are explicitly enabled. This means these payment methods will be shown on checkout. Example use case is to only enable a specific payment method like cashapp, or extending the platform defaults with additional methods.
    #[serde(default)]
    pub enabled: Vec<PaymentMethodTypes>,
    /// Whether Whop's platform default payment method enablement settings are included in this configuration. The full list of default payment methods can be found in the documentation at docs.whop.com/payments.
    #[serde(default)]
    pub include_platform_defaults: bool,
}

impl CheckoutConfigurationListItemPaymentMethodConfiguration {
    pub fn builder() -> CheckoutConfigurationListItemPaymentMethodConfigurationBuilder {
        <CheckoutConfigurationListItemPaymentMethodConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutConfigurationListItemPaymentMethodConfigurationBuilder {
    disabled: Option<Vec<PaymentMethodTypes>>,
    enabled: Option<Vec<PaymentMethodTypes>>,
    include_platform_defaults: Option<bool>,
}

impl CheckoutConfigurationListItemPaymentMethodConfigurationBuilder {
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

    /// Consumes the builder and constructs a [`CheckoutConfigurationListItemPaymentMethodConfiguration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`disabled`](CheckoutConfigurationListItemPaymentMethodConfigurationBuilder::disabled)
    /// - [`enabled`](CheckoutConfigurationListItemPaymentMethodConfigurationBuilder::enabled)
    /// - [`include_platform_defaults`](CheckoutConfigurationListItemPaymentMethodConfigurationBuilder::include_platform_defaults)
    pub fn build(
        self,
    ) -> Result<CheckoutConfigurationListItemPaymentMethodConfiguration, BuildError> {
        Ok(CheckoutConfigurationListItemPaymentMethodConfiguration {
            disabled: self
                .disabled
                .ok_or_else(|| BuildError::missing_field("disabled"))?,
            enabled: self
                .enabled
                .ok_or_else(|| BuildError::missing_field("enabled"))?,
            include_platform_defaults: self
                .include_platform_defaults
                .ok_or_else(|| BuildError::missing_field("include_platform_defaults"))?,
        })
    }
}
