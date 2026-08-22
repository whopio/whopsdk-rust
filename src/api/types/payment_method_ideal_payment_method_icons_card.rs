pub use crate::prelude::*;

/// The credit-card-proportioned tile (48x30).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodIdealPaymentMethodIconsCard {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodIdealPaymentMethodIconsCardDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodIdealPaymentMethodIconsCardLight,
}

impl PaymentMethodIdealPaymentMethodIconsCard {
    pub fn builder() -> PaymentMethodIdealPaymentMethodIconsCardBuilder {
        <PaymentMethodIdealPaymentMethodIconsCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodIdealPaymentMethodIconsCardBuilder {
    dark: Option<PaymentMethodIdealPaymentMethodIconsCardDark>,
    light: Option<PaymentMethodIdealPaymentMethodIconsCardLight>,
}

impl PaymentMethodIdealPaymentMethodIconsCardBuilder {
    pub fn dark(mut self, value: PaymentMethodIdealPaymentMethodIconsCardDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodIdealPaymentMethodIconsCardLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodIdealPaymentMethodIconsCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodIdealPaymentMethodIconsCardBuilder::dark)
    /// - [`light`](PaymentMethodIdealPaymentMethodIconsCardBuilder::light)
    pub fn build(self) -> Result<PaymentMethodIdealPaymentMethodIconsCard, BuildError> {
        Ok(PaymentMethodIdealPaymentMethodIconsCard {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
