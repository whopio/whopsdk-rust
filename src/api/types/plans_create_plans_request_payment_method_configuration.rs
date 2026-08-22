pub use crate::prelude::*;

/// Explicit payment method configuration for the plan. When not provided, the account's defaults apply.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePlansRequestPaymentMethodConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_platform_defaults: Option<bool>,
}

impl CreatePlansRequestPaymentMethodConfiguration {
    pub fn builder() -> CreatePlansRequestPaymentMethodConfigurationBuilder {
        <CreatePlansRequestPaymentMethodConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePlansRequestPaymentMethodConfigurationBuilder {
    disabled: Option<Vec<String>>,
    enabled: Option<Vec<String>>,
    include_platform_defaults: Option<bool>,
}

impl CreatePlansRequestPaymentMethodConfigurationBuilder {
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

    /// Consumes the builder and constructs a [`CreatePlansRequestPaymentMethodConfiguration`].
    pub fn build(self) -> Result<CreatePlansRequestPaymentMethodConfiguration, BuildError> {
        Ok(CreatePlansRequestPaymentMethodConfiguration {
            disabled: self.disabled,
            enabled: self.enabled,
            include_platform_defaults: self.include_platform_defaults,
        })
    }
}
