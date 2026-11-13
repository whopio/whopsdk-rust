pub use crate::prelude::*;

/// Explicit payment method configuration for the variant. When not provided, the account's defaults apply. Send at least one of `enabled` or `disabled`; an omitted one is empty.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateVariantsRequestPaymentMethodConfiguration {
    /// Payment method types explicitly disabled for this variant — the `type` values from the payment method types catalogue. Types Whop no longer offers, and the read-only `unknown` placeholder, are dropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<Vec<PaymentMethodTypes>>,
    /// Payment method types explicitly enabled for this variant — the `type` values from the payment method types catalogue. Types Whop no longer offers, and the read-only `unknown` placeholder, are dropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Vec<PaymentMethodTypes>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_platform_defaults: Option<bool>,
}

impl CreateVariantsRequestPaymentMethodConfiguration {
    pub fn builder() -> CreateVariantsRequestPaymentMethodConfigurationBuilder {
        <CreateVariantsRequestPaymentMethodConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateVariantsRequestPaymentMethodConfigurationBuilder {
    disabled: Option<Vec<PaymentMethodTypes>>,
    enabled: Option<Vec<PaymentMethodTypes>>,
    include_platform_defaults: Option<bool>,
}

impl CreateVariantsRequestPaymentMethodConfigurationBuilder {
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

    /// Consumes the builder and constructs a [`CreateVariantsRequestPaymentMethodConfiguration`].
    pub fn build(self) -> Result<CreateVariantsRequestPaymentMethodConfiguration, BuildError> {
        Ok(CreateVariantsRequestPaymentMethodConfiguration {
            disabled: self.disabled,
            enabled: self.enabled,
            include_platform_defaults: self.include_platform_defaults,
        })
    }
}
