pub use crate::prelude::*;

/// The credit-card-proportioned tile (48x30).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemIdealPaymentMethodIconsCard {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodListItemIdealPaymentMethodIconsCardDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodListItemIdealPaymentMethodIconsCardLight,
}

impl PaymentMethodListItemIdealPaymentMethodIconsCard {
    pub fn builder() -> PaymentMethodListItemIdealPaymentMethodIconsCardBuilder {
        <PaymentMethodListItemIdealPaymentMethodIconsCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemIdealPaymentMethodIconsCardBuilder {
    dark: Option<PaymentMethodListItemIdealPaymentMethodIconsCardDark>,
    light: Option<PaymentMethodListItemIdealPaymentMethodIconsCardLight>,
}

impl PaymentMethodListItemIdealPaymentMethodIconsCardBuilder {
    pub fn dark(mut self, value: PaymentMethodListItemIdealPaymentMethodIconsCardDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodListItemIdealPaymentMethodIconsCardLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemIdealPaymentMethodIconsCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodListItemIdealPaymentMethodIconsCardBuilder::dark)
    /// - [`light`](PaymentMethodListItemIdealPaymentMethodIconsCardBuilder::light)
    pub fn build(self) -> Result<PaymentMethodListItemIdealPaymentMethodIconsCard, BuildError> {
        Ok(PaymentMethodListItemIdealPaymentMethodIconsCard {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
