pub use crate::prelude::*;

/// The square tile (32x32).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemSepaDebitPaymentMethodIconsSquare {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodListItemSepaDebitPaymentMethodIconsSquareDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodListItemSepaDebitPaymentMethodIconsSquareLight,
}

impl PaymentMethodListItemSepaDebitPaymentMethodIconsSquare {
    pub fn builder() -> PaymentMethodListItemSepaDebitPaymentMethodIconsSquareBuilder {
        <PaymentMethodListItemSepaDebitPaymentMethodIconsSquareBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemSepaDebitPaymentMethodIconsSquareBuilder {
    dark: Option<PaymentMethodListItemSepaDebitPaymentMethodIconsSquareDark>,
    light: Option<PaymentMethodListItemSepaDebitPaymentMethodIconsSquareLight>,
}

impl PaymentMethodListItemSepaDebitPaymentMethodIconsSquareBuilder {
    pub fn dark(
        mut self,
        value: PaymentMethodListItemSepaDebitPaymentMethodIconsSquareDark,
    ) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(
        mut self,
        value: PaymentMethodListItemSepaDebitPaymentMethodIconsSquareLight,
    ) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemSepaDebitPaymentMethodIconsSquare`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodListItemSepaDebitPaymentMethodIconsSquareBuilder::dark)
    /// - [`light`](PaymentMethodListItemSepaDebitPaymentMethodIconsSquareBuilder::light)
    pub fn build(
        self,
    ) -> Result<PaymentMethodListItemSepaDebitPaymentMethodIconsSquare, BuildError> {
        Ok(PaymentMethodListItemSepaDebitPaymentMethodIconsSquare {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
