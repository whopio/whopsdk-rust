pub use crate::prelude::*;

/// Explicit payment method configuration for the plan. When not provided, the account's defaults apply.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePlansRequestPaymentMethodConfiguration {
    /// Payment method types explicitly disabled for this plan — the `type` values from the payment method types catalogue. Types Whop no longer offers, and the read-only `unknown` placeholder, are dropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<Vec<PaymentMethodTypes>>,
    /// Payment method types explicitly enabled for this plan — the `type` values from the payment method types catalogue. Types Whop no longer offers, and the read-only `unknown` placeholder, are dropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Vec<PaymentMethodTypes>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_platform_defaults: Option<bool>,
}

impl UpdatePlansRequestPaymentMethodConfiguration {
    pub fn builder() -> UpdatePlansRequestPaymentMethodConfigurationBuilder {
        <UpdatePlansRequestPaymentMethodConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePlansRequestPaymentMethodConfigurationBuilder {
    disabled: Option<Vec<PaymentMethodTypes>>,
    enabled: Option<Vec<PaymentMethodTypes>>,
    include_platform_defaults: Option<bool>,
}

impl UpdatePlansRequestPaymentMethodConfigurationBuilder {
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

    /// Consumes the builder and constructs a [`UpdatePlansRequestPaymentMethodConfiguration`].
    pub fn build(self) -> Result<UpdatePlansRequestPaymentMethodConfiguration, BuildError> {
        Ok(UpdatePlansRequestPaymentMethodConfiguration {
            disabled: self.disabled,
            enabled: self.enabled,
            include_platform_defaults: self.include_platform_defaults,
        })
    }
}
