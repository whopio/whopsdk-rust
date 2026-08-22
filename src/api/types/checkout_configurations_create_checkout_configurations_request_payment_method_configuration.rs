pub use crate::prelude::*;

/// Payment method overrides for this checkout. `null` uses the plan or platform defaults.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCheckoutConfigurationsRequestPaymentMethodConfiguration {
    /// Payment methods explicitly disabled for checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<Vec<String>>,
    /// Payment methods explicitly enabled for checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Vec<String>>,
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
    disabled: Option<Vec<String>>,
    enabled: Option<Vec<String>>,
    include_platform_defaults: Option<bool>,
}

impl CreateCheckoutConfigurationsRequestPaymentMethodConfigurationBuilder {
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
