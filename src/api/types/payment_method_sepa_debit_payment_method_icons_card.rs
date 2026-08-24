pub use crate::prelude::*;

/// The credit-card-proportioned tile (48x30).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodSepaDebitPaymentMethodIconsCard {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodSepaDebitPaymentMethodIconsCardDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodSepaDebitPaymentMethodIconsCardLight,
}

impl PaymentMethodSepaDebitPaymentMethodIconsCard {
    pub fn builder() -> PaymentMethodSepaDebitPaymentMethodIconsCardBuilder {
        <PaymentMethodSepaDebitPaymentMethodIconsCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodSepaDebitPaymentMethodIconsCardBuilder {
    dark: Option<PaymentMethodSepaDebitPaymentMethodIconsCardDark>,
    light: Option<PaymentMethodSepaDebitPaymentMethodIconsCardLight>,
}

impl PaymentMethodSepaDebitPaymentMethodIconsCardBuilder {
    pub fn dark(mut self, value: PaymentMethodSepaDebitPaymentMethodIconsCardDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodSepaDebitPaymentMethodIconsCardLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodSepaDebitPaymentMethodIconsCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodSepaDebitPaymentMethodIconsCardBuilder::dark)
    /// - [`light`](PaymentMethodSepaDebitPaymentMethodIconsCardBuilder::light)
    pub fn build(self) -> Result<PaymentMethodSepaDebitPaymentMethodIconsCard, BuildError> {
        Ok(PaymentMethodSepaDebitPaymentMethodIconsCard {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
