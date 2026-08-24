pub use crate::prelude::*;

/// The credit-card-proportioned tile (48x30).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemBasePaymentMethodIconsCard {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodListItemBasePaymentMethodIconsCardDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodListItemBasePaymentMethodIconsCardLight,
}

impl PaymentMethodListItemBasePaymentMethodIconsCard {
    pub fn builder() -> PaymentMethodListItemBasePaymentMethodIconsCardBuilder {
        <PaymentMethodListItemBasePaymentMethodIconsCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemBasePaymentMethodIconsCardBuilder {
    dark: Option<PaymentMethodListItemBasePaymentMethodIconsCardDark>,
    light: Option<PaymentMethodListItemBasePaymentMethodIconsCardLight>,
}

impl PaymentMethodListItemBasePaymentMethodIconsCardBuilder {
    pub fn dark(mut self, value: PaymentMethodListItemBasePaymentMethodIconsCardDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodListItemBasePaymentMethodIconsCardLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemBasePaymentMethodIconsCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodListItemBasePaymentMethodIconsCardBuilder::dark)
    /// - [`light`](PaymentMethodListItemBasePaymentMethodIconsCardBuilder::light)
    pub fn build(self) -> Result<PaymentMethodListItemBasePaymentMethodIconsCard, BuildError> {
        Ok(PaymentMethodListItemBasePaymentMethodIconsCard {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
