pub use crate::prelude::*;

/// The credit-card-proportioned tile (48x30).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodCashappPaymentMethodIconsCard {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodCashappPaymentMethodIconsCardDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodCashappPaymentMethodIconsCardLight,
}

impl PaymentMethodCashappPaymentMethodIconsCard {
    pub fn builder() -> PaymentMethodCashappPaymentMethodIconsCardBuilder {
        <PaymentMethodCashappPaymentMethodIconsCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodCashappPaymentMethodIconsCardBuilder {
    dark: Option<PaymentMethodCashappPaymentMethodIconsCardDark>,
    light: Option<PaymentMethodCashappPaymentMethodIconsCardLight>,
}

impl PaymentMethodCashappPaymentMethodIconsCardBuilder {
    pub fn dark(mut self, value: PaymentMethodCashappPaymentMethodIconsCardDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodCashappPaymentMethodIconsCardLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodCashappPaymentMethodIconsCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodCashappPaymentMethodIconsCardBuilder::dark)
    /// - [`light`](PaymentMethodCashappPaymentMethodIconsCardBuilder::light)
    pub fn build(self) -> Result<PaymentMethodCashappPaymentMethodIconsCard, BuildError> {
        Ok(PaymentMethodCashappPaymentMethodIconsCard {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
