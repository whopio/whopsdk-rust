pub use crate::prelude::*;

/// The credit-card-proportioned tile (48x30).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodPlatformBalancePaymentMethodIconsCard {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodPlatformBalancePaymentMethodIconsCardDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodPlatformBalancePaymentMethodIconsCardLight,
}

impl PaymentMethodPlatformBalancePaymentMethodIconsCard {
    pub fn builder() -> PaymentMethodPlatformBalancePaymentMethodIconsCardBuilder {
        <PaymentMethodPlatformBalancePaymentMethodIconsCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodPlatformBalancePaymentMethodIconsCardBuilder {
    dark: Option<PaymentMethodPlatformBalancePaymentMethodIconsCardDark>,
    light: Option<PaymentMethodPlatformBalancePaymentMethodIconsCardLight>,
}

impl PaymentMethodPlatformBalancePaymentMethodIconsCardBuilder {
    pub fn dark(mut self, value: PaymentMethodPlatformBalancePaymentMethodIconsCardDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodPlatformBalancePaymentMethodIconsCardLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodPlatformBalancePaymentMethodIconsCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodPlatformBalancePaymentMethodIconsCardBuilder::dark)
    /// - [`light`](PaymentMethodPlatformBalancePaymentMethodIconsCardBuilder::light)
    pub fn build(self) -> Result<PaymentMethodPlatformBalancePaymentMethodIconsCard, BuildError> {
        Ok(PaymentMethodPlatformBalancePaymentMethodIconsCard {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
