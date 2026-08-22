pub use crate::prelude::*;

/// The company's logo.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccountLogo {
    /// A pre-optimized URL for rendering this attachment on the client. This should be used for displaying attachments in apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccountLogo {
    pub fn builder() -> PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccountLogoBuilder {
        <PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccountLogoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccountLogoBuilder {
    url: Option<String>,
}

impl PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccountLogoBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccountLogo`].
    pub fn build(
        self,
    ) -> Result<PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccountLogo, BuildError>
    {
        Ok(PaymentMethodPlatformBalancePaymentMethodPlatformBalanceAccountLogo { url: self.url })
    }
}
