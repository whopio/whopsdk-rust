pub use crate::prelude::*;

/// The credit-card-proportioned tile (48x30).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemCardPaymentMethodIconsCard {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodListItemCardPaymentMethodIconsCardDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodListItemCardPaymentMethodIconsCardLight,
}

impl PaymentMethodListItemCardPaymentMethodIconsCard {
    pub fn builder() -> PaymentMethodListItemCardPaymentMethodIconsCardBuilder {
        <PaymentMethodListItemCardPaymentMethodIconsCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemCardPaymentMethodIconsCardBuilder {
    dark: Option<PaymentMethodListItemCardPaymentMethodIconsCardDark>,
    light: Option<PaymentMethodListItemCardPaymentMethodIconsCardLight>,
}

impl PaymentMethodListItemCardPaymentMethodIconsCardBuilder {
    pub fn dark(mut self, value: PaymentMethodListItemCardPaymentMethodIconsCardDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodListItemCardPaymentMethodIconsCardLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemCardPaymentMethodIconsCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodListItemCardPaymentMethodIconsCardBuilder::dark)
    /// - [`light`](PaymentMethodListItemCardPaymentMethodIconsCardBuilder::light)
    pub fn build(self) -> Result<PaymentMethodListItemCardPaymentMethodIconsCard, BuildError> {
        Ok(PaymentMethodListItemCardPaymentMethodIconsCard {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
