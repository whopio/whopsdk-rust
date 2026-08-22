pub use crate::prelude::*;

/// The credit-card-proportioned tile (48x30).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodUsBankAccountPaymentMethodIconsCard {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodUsBankAccountPaymentMethodIconsCardDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodUsBankAccountPaymentMethodIconsCardLight,
}

impl PaymentMethodUsBankAccountPaymentMethodIconsCard {
    pub fn builder() -> PaymentMethodUsBankAccountPaymentMethodIconsCardBuilder {
        <PaymentMethodUsBankAccountPaymentMethodIconsCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodUsBankAccountPaymentMethodIconsCardBuilder {
    dark: Option<PaymentMethodUsBankAccountPaymentMethodIconsCardDark>,
    light: Option<PaymentMethodUsBankAccountPaymentMethodIconsCardLight>,
}

impl PaymentMethodUsBankAccountPaymentMethodIconsCardBuilder {
    pub fn dark(mut self, value: PaymentMethodUsBankAccountPaymentMethodIconsCardDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodUsBankAccountPaymentMethodIconsCardLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodUsBankAccountPaymentMethodIconsCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodUsBankAccountPaymentMethodIconsCardBuilder::dark)
    /// - [`light`](PaymentMethodUsBankAccountPaymentMethodIconsCardBuilder::light)
    pub fn build(self) -> Result<PaymentMethodUsBankAccountPaymentMethodIconsCard, BuildError> {
        Ok(PaymentMethodUsBankAccountPaymentMethodIconsCard {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
