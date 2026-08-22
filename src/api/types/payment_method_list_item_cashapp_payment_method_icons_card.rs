pub use crate::prelude::*;

/// The credit-card-proportioned tile (48x30).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemCashappPaymentMethodIconsCard {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodListItemCashappPaymentMethodIconsCardDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodListItemCashappPaymentMethodIconsCardLight,
}

impl PaymentMethodListItemCashappPaymentMethodIconsCard {
    pub fn builder() -> PaymentMethodListItemCashappPaymentMethodIconsCardBuilder {
        <PaymentMethodListItemCashappPaymentMethodIconsCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemCashappPaymentMethodIconsCardBuilder {
    dark: Option<PaymentMethodListItemCashappPaymentMethodIconsCardDark>,
    light: Option<PaymentMethodListItemCashappPaymentMethodIconsCardLight>,
}

impl PaymentMethodListItemCashappPaymentMethodIconsCardBuilder {
    pub fn dark(mut self, value: PaymentMethodListItemCashappPaymentMethodIconsCardDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodListItemCashappPaymentMethodIconsCardLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemCashappPaymentMethodIconsCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodListItemCashappPaymentMethodIconsCardBuilder::dark)
    /// - [`light`](PaymentMethodListItemCashappPaymentMethodIconsCardBuilder::light)
    pub fn build(self) -> Result<PaymentMethodListItemCashappPaymentMethodIconsCard, BuildError> {
        Ok(PaymentMethodListItemCashappPaymentMethodIconsCard {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
