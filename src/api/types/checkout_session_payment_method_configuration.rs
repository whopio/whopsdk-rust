pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckoutSessionPaymentMethodConfiguration {
    #[serde(default)]
    pub disabled: Vec<String>,
    #[serde(default)]
    pub enabled: Vec<String>,
    /// Whether Whop's default set is the starting point. When `false`, only `enabled` is offered.
    #[serde(default)]
    pub include_platform_defaults: bool,
}

impl CheckoutSessionPaymentMethodConfiguration {
    pub fn builder() -> CheckoutSessionPaymentMethodConfigurationBuilder {
        <CheckoutSessionPaymentMethodConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionPaymentMethodConfigurationBuilder {
    disabled: Option<Vec<String>>,
    enabled: Option<Vec<String>>,
    include_platform_defaults: Option<bool>,
}

impl CheckoutSessionPaymentMethodConfigurationBuilder {
    pub fn disabled(mut self, value: Vec<String>) -> Self {
        self.disabled = Some(value);
        self
    }

    pub fn enabled(mut self, value: Vec<String>) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn include_platform_defaults(mut self, value: bool) -> Self {
        self.include_platform_defaults = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionPaymentMethodConfiguration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`disabled`](CheckoutSessionPaymentMethodConfigurationBuilder::disabled)
    /// - [`enabled`](CheckoutSessionPaymentMethodConfigurationBuilder::enabled)
    /// - [`include_platform_defaults`](CheckoutSessionPaymentMethodConfigurationBuilder::include_platform_defaults)
    pub fn build(self) -> Result<CheckoutSessionPaymentMethodConfiguration, BuildError> {
        Ok(CheckoutSessionPaymentMethodConfiguration {
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
