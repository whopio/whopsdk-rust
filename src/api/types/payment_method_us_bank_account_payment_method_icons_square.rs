pub use crate::prelude::*;

/// The square tile (32x32).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodUsBankAccountPaymentMethodIconsSquare {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodUsBankAccountPaymentMethodIconsSquareDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodUsBankAccountPaymentMethodIconsSquareLight,
}

impl PaymentMethodUsBankAccountPaymentMethodIconsSquare {
    pub fn builder() -> PaymentMethodUsBankAccountPaymentMethodIconsSquareBuilder {
        <PaymentMethodUsBankAccountPaymentMethodIconsSquareBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodUsBankAccountPaymentMethodIconsSquareBuilder {
    dark: Option<PaymentMethodUsBankAccountPaymentMethodIconsSquareDark>,
    light: Option<PaymentMethodUsBankAccountPaymentMethodIconsSquareLight>,
}

impl PaymentMethodUsBankAccountPaymentMethodIconsSquareBuilder {
    pub fn dark(mut self, value: PaymentMethodUsBankAccountPaymentMethodIconsSquareDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodUsBankAccountPaymentMethodIconsSquareLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodUsBankAccountPaymentMethodIconsSquare`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodUsBankAccountPaymentMethodIconsSquareBuilder::dark)
    /// - [`light`](PaymentMethodUsBankAccountPaymentMethodIconsSquareBuilder::light)
    pub fn build(self) -> Result<PaymentMethodUsBankAccountPaymentMethodIconsSquare, BuildError> {
        Ok(PaymentMethodUsBankAccountPaymentMethodIconsSquare {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
