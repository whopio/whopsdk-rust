pub use crate::prelude::*;

/// The credit-card-proportioned tile (48x30).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemUsBankAccountPaymentMethodIconsCard {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodListItemUsBankAccountPaymentMethodIconsCardDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodListItemUsBankAccountPaymentMethodIconsCardLight,
}

impl PaymentMethodListItemUsBankAccountPaymentMethodIconsCard {
    pub fn builder() -> PaymentMethodListItemUsBankAccountPaymentMethodIconsCardBuilder {
        <PaymentMethodListItemUsBankAccountPaymentMethodIconsCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemUsBankAccountPaymentMethodIconsCardBuilder {
    dark: Option<PaymentMethodListItemUsBankAccountPaymentMethodIconsCardDark>,
    light: Option<PaymentMethodListItemUsBankAccountPaymentMethodIconsCardLight>,
}

impl PaymentMethodListItemUsBankAccountPaymentMethodIconsCardBuilder {
    pub fn dark(
        mut self,
        value: PaymentMethodListItemUsBankAccountPaymentMethodIconsCardDark,
    ) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(
        mut self,
        value: PaymentMethodListItemUsBankAccountPaymentMethodIconsCardLight,
    ) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemUsBankAccountPaymentMethodIconsCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodListItemUsBankAccountPaymentMethodIconsCardBuilder::dark)
    /// - [`light`](PaymentMethodListItemUsBankAccountPaymentMethodIconsCardBuilder::light)
    pub fn build(
        self,
    ) -> Result<PaymentMethodListItemUsBankAccountPaymentMethodIconsCard, BuildError> {
        Ok(PaymentMethodListItemUsBankAccountPaymentMethodIconsCard {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
