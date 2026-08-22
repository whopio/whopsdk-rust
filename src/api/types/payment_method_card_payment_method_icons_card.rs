pub use crate::prelude::*;

/// The credit-card-proportioned tile (48x30).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodCardPaymentMethodIconsCard {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodCardPaymentMethodIconsCardDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodCardPaymentMethodIconsCardLight,
}

impl PaymentMethodCardPaymentMethodIconsCard {
    pub fn builder() -> PaymentMethodCardPaymentMethodIconsCardBuilder {
        <PaymentMethodCardPaymentMethodIconsCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodCardPaymentMethodIconsCardBuilder {
    dark: Option<PaymentMethodCardPaymentMethodIconsCardDark>,
    light: Option<PaymentMethodCardPaymentMethodIconsCardLight>,
}

impl PaymentMethodCardPaymentMethodIconsCardBuilder {
    pub fn dark(mut self, value: PaymentMethodCardPaymentMethodIconsCardDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodCardPaymentMethodIconsCardLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodCardPaymentMethodIconsCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodCardPaymentMethodIconsCardBuilder::dark)
    /// - [`light`](PaymentMethodCardPaymentMethodIconsCardBuilder::light)
    pub fn build(self) -> Result<PaymentMethodCardPaymentMethodIconsCard, BuildError> {
        Ok(PaymentMethodCardPaymentMethodIconsCard {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
