pub use crate::prelude::*;

/// The company's logo.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceAccountLogo {
    /// A pre-optimized URL for rendering this attachment on the client. This should be used for displaying attachments in apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceAccountLogo {
    pub fn builder(
    ) -> PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceAccountLogoBuilder {
        <PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceAccountLogoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceAccountLogoBuilder {
    url: Option<String>,
}

impl PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceAccountLogoBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceAccountLogo`].
    pub fn build(
        self,
    ) -> Result<
        PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceAccountLogo,
        BuildError,
    > {
        Ok(
            PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalanceAccountLogo {
                url: self.url,
            },
        )
    }
}
