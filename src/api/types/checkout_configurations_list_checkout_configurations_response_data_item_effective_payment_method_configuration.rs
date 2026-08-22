pub use crate::prelude::*;

/// The configuration governing a checkout mounted from this configuration, resolved through every layer (its own overrides, the plan's, and the account's) — the shape a session's `payment_method_configuration` carries. Apply it over the payment method types catalogue for the offerable set. `null` means platform defaults; `payment_method_configuration` stays this configuration's own editable override.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCheckoutConfigurationsResponseDataItemEffectivePaymentMethodConfiguration {
    /// Payment methods explicitly disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<Vec<String>>,
    /// Payment methods explicitly enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Vec<String>>,
    /// Whether platform default payment methods are included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_platform_defaults: Option<bool>,
}

impl ListCheckoutConfigurationsResponseDataItemEffectivePaymentMethodConfiguration {
    pub fn builder(
    ) -> ListCheckoutConfigurationsResponseDataItemEffectivePaymentMethodConfigurationBuilder {
        <ListCheckoutConfigurationsResponseDataItemEffectivePaymentMethodConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCheckoutConfigurationsResponseDataItemEffectivePaymentMethodConfigurationBuilder {
    disabled: Option<Vec<String>>,
    enabled: Option<Vec<String>>,
    include_platform_defaults: Option<bool>,
}

impl ListCheckoutConfigurationsResponseDataItemEffectivePaymentMethodConfigurationBuilder {
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

    /// Consumes the builder and constructs a [`ListCheckoutConfigurationsResponseDataItemEffectivePaymentMethodConfiguration`].
    pub fn build(
        self,
    ) -> Result<
        ListCheckoutConfigurationsResponseDataItemEffectivePaymentMethodConfiguration,
        BuildError,
    > {
        Ok(
            ListCheckoutConfigurationsResponseDataItemEffectivePaymentMethodConfiguration {
                disabled: self.disabled,
                enabled: self.enabled,
                include_platform_defaults: self.include_platform_defaults,
            },
        )
    }
}
