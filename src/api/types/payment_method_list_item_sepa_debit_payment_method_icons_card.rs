pub use crate::prelude::*;

/// The credit-card-proportioned tile (48x30).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemSepaDebitPaymentMethodIconsCard {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodListItemSepaDebitPaymentMethodIconsCardDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodListItemSepaDebitPaymentMethodIconsCardLight,
}

impl PaymentMethodListItemSepaDebitPaymentMethodIconsCard {
    pub fn builder() -> PaymentMethodListItemSepaDebitPaymentMethodIconsCardBuilder {
        <PaymentMethodListItemSepaDebitPaymentMethodIconsCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemSepaDebitPaymentMethodIconsCardBuilder {
    dark: Option<PaymentMethodListItemSepaDebitPaymentMethodIconsCardDark>,
    light: Option<PaymentMethodListItemSepaDebitPaymentMethodIconsCardLight>,
}

impl PaymentMethodListItemSepaDebitPaymentMethodIconsCardBuilder {
    pub fn dark(mut self, value: PaymentMethodListItemSepaDebitPaymentMethodIconsCardDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(
        mut self,
        value: PaymentMethodListItemSepaDebitPaymentMethodIconsCardLight,
    ) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemSepaDebitPaymentMethodIconsCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodListItemSepaDebitPaymentMethodIconsCardBuilder::dark)
    /// - [`light`](PaymentMethodListItemSepaDebitPaymentMethodIconsCardBuilder::light)
    pub fn build(self) -> Result<PaymentMethodListItemSepaDebitPaymentMethodIconsCard, BuildError> {
        Ok(PaymentMethodListItemSepaDebitPaymentMethodIconsCard {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
