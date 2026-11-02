pub use crate::prelude::*;

/// The explicit payment method configuration for the plan. If not provided, the platform or company's defaults will apply.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateInvoicesRequestBodyProductIdPlanPaymentMethodConfiguration {
    /// An array of payment method identifiers that are explicitly disabled. Only applies if the include_platform_defaults is true.
    #[serde(default)]
    pub disabled: Vec<PaymentMethodTypes>,
    /// An array of payment method identifiers that are explicitly enabled. This means these payment methods will be shown on checkout. Example use case is to only enable a specific payment method like cashapp, or extending the platform defaults with additional methods.
    #[serde(default)]
    pub enabled: Vec<PaymentMethodTypes>,
    /// Whether Whop's platform default payment method enablement settings are included in this configuration. The full list of default payment methods can be found in the documentation at docs.whop.com/payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_platform_defaults: Option<bool>,
}

impl CreateInvoicesRequestBodyProductIdPlanPaymentMethodConfiguration {
    pub fn builder() -> CreateInvoicesRequestBodyProductIdPlanPaymentMethodConfigurationBuilder {
        <CreateInvoicesRequestBodyProductIdPlanPaymentMethodConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateInvoicesRequestBodyProductIdPlanPaymentMethodConfigurationBuilder {
    disabled: Option<Vec<PaymentMethodTypes>>,
    enabled: Option<Vec<PaymentMethodTypes>>,
    include_platform_defaults: Option<bool>,
}

impl CreateInvoicesRequestBodyProductIdPlanPaymentMethodConfigurationBuilder {
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

    /// Consumes the builder and constructs a [`CreateInvoicesRequestBodyProductIdPlanPaymentMethodConfiguration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`disabled`](CreateInvoicesRequestBodyProductIdPlanPaymentMethodConfigurationBuilder::disabled)
    /// - [`enabled`](CreateInvoicesRequestBodyProductIdPlanPaymentMethodConfigurationBuilder::enabled)
    pub fn build(
        self,
    ) -> Result<CreateInvoicesRequestBodyProductIdPlanPaymentMethodConfiguration, BuildError> {
        Ok(
            CreateInvoicesRequestBodyProductIdPlanPaymentMethodConfiguration {
                disabled: self
                    .disabled
                    .ok_or_else(|| BuildError::missing_field("disabled"))?,
                enabled: self
                    .enabled
                    .ok_or_else(|| BuildError::missing_field("enabled"))?,
                include_platform_defaults: self.include_platform_defaults,
            },
        )
    }
}
