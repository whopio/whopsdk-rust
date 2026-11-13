pub use crate::prelude::*;

/// The payment methods the customer can choose from when paying through the invoice's checkout link, in the shape a variant or checkout configuration takes. It takes precedence over the invoice's variants and the account, and changes neither. Send both lists. `null` uses its variants' payment methods. Can be updated on drafts.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateInvoicesRequestPaymentMethodConfiguration {
    /// Payment method types explicitly disabled for this invoice. Use the `type` values from the payment method types catalogue. Types Whop no longer offers, and the read-only `unknown` placeholder, are dropped.
    #[serde(default)]
    pub disabled: Vec<PaymentMethodTypes>,
    /// Payment method types explicitly enabled for this invoice. Use the `type` values from the payment method types catalogue. Types Whop no longer offers, and the read-only `unknown` placeholder, are dropped.
    #[serde(default)]
    pub enabled: Vec<PaymentMethodTypes>,
    /// Whether Whop's default set is the starting point. When `false`, only `enabled` is offered. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_platform_defaults: Option<bool>,
}

impl UpdateInvoicesRequestPaymentMethodConfiguration {
    pub fn builder() -> UpdateInvoicesRequestPaymentMethodConfigurationBuilder {
        <UpdateInvoicesRequestPaymentMethodConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateInvoicesRequestPaymentMethodConfigurationBuilder {
    disabled: Option<Vec<PaymentMethodTypes>>,
    enabled: Option<Vec<PaymentMethodTypes>>,
    include_platform_defaults: Option<bool>,
}

impl UpdateInvoicesRequestPaymentMethodConfigurationBuilder {
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

    /// Consumes the builder and constructs a [`UpdateInvoicesRequestPaymentMethodConfiguration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`disabled`](UpdateInvoicesRequestPaymentMethodConfigurationBuilder::disabled)
    /// - [`enabled`](UpdateInvoicesRequestPaymentMethodConfigurationBuilder::enabled)
    pub fn build(self) -> Result<UpdateInvoicesRequestPaymentMethodConfiguration, BuildError> {
        Ok(UpdateInvoicesRequestPaymentMethodConfiguration {
            disabled: self
                .disabled
                .ok_or_else(|| BuildError::missing_field("disabled"))?,
            enabled: self
                .enabled
                .ok_or_else(|| BuildError::missing_field("enabled"))?,
            include_platform_defaults: self.include_platform_defaults,
        })
    }
}
