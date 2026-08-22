pub use crate::prelude::*;

/// The square tile (32x32).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodSepaDebitPaymentMethodIconsSquare {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodSepaDebitPaymentMethodIconsSquareDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodSepaDebitPaymentMethodIconsSquareLight,
}

impl PaymentMethodSepaDebitPaymentMethodIconsSquare {
    pub fn builder() -> PaymentMethodSepaDebitPaymentMethodIconsSquareBuilder {
        <PaymentMethodSepaDebitPaymentMethodIconsSquareBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodSepaDebitPaymentMethodIconsSquareBuilder {
    dark: Option<PaymentMethodSepaDebitPaymentMethodIconsSquareDark>,
    light: Option<PaymentMethodSepaDebitPaymentMethodIconsSquareLight>,
}

impl PaymentMethodSepaDebitPaymentMethodIconsSquareBuilder {
    pub fn dark(mut self, value: PaymentMethodSepaDebitPaymentMethodIconsSquareDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodSepaDebitPaymentMethodIconsSquareLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodSepaDebitPaymentMethodIconsSquare`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodSepaDebitPaymentMethodIconsSquareBuilder::dark)
    /// - [`light`](PaymentMethodSepaDebitPaymentMethodIconsSquareBuilder::light)
    pub fn build(self) -> Result<PaymentMethodSepaDebitPaymentMethodIconsSquare, BuildError> {
        Ok(PaymentMethodSepaDebitPaymentMethodIconsSquare {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
