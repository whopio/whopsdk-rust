pub use crate::prelude::*;

/// The credit-card-proportioned tile (48x30).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodBasePaymentMethodIconsCard {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodBasePaymentMethodIconsCardDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodBasePaymentMethodIconsCardLight,
}

impl PaymentMethodBasePaymentMethodIconsCard {
    pub fn builder() -> PaymentMethodBasePaymentMethodIconsCardBuilder {
        <PaymentMethodBasePaymentMethodIconsCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodBasePaymentMethodIconsCardBuilder {
    dark: Option<PaymentMethodBasePaymentMethodIconsCardDark>,
    light: Option<PaymentMethodBasePaymentMethodIconsCardLight>,
}

impl PaymentMethodBasePaymentMethodIconsCardBuilder {
    pub fn dark(mut self, value: PaymentMethodBasePaymentMethodIconsCardDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodBasePaymentMethodIconsCardLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodBasePaymentMethodIconsCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodBasePaymentMethodIconsCardBuilder::dark)
    /// - [`light`](PaymentMethodBasePaymentMethodIconsCardBuilder::light)
    pub fn build(self) -> Result<PaymentMethodBasePaymentMethodIconsCard, BuildError> {
        Ok(PaymentMethodBasePaymentMethodIconsCard {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
