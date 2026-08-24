pub use crate::prelude::*;

/// The square tile (32x32).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodCashappPaymentMethodIconsSquare {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodCashappPaymentMethodIconsSquareDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodCashappPaymentMethodIconsSquareLight,
}

impl PaymentMethodCashappPaymentMethodIconsSquare {
    pub fn builder() -> PaymentMethodCashappPaymentMethodIconsSquareBuilder {
        <PaymentMethodCashappPaymentMethodIconsSquareBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodCashappPaymentMethodIconsSquareBuilder {
    dark: Option<PaymentMethodCashappPaymentMethodIconsSquareDark>,
    light: Option<PaymentMethodCashappPaymentMethodIconsSquareLight>,
}

impl PaymentMethodCashappPaymentMethodIconsSquareBuilder {
    pub fn dark(mut self, value: PaymentMethodCashappPaymentMethodIconsSquareDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodCashappPaymentMethodIconsSquareLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodCashappPaymentMethodIconsSquare`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodCashappPaymentMethodIconsSquareBuilder::dark)
    /// - [`light`](PaymentMethodCashappPaymentMethodIconsSquareBuilder::light)
    pub fn build(self) -> Result<PaymentMethodCashappPaymentMethodIconsSquare, BuildError> {
        Ok(PaymentMethodCashappPaymentMethodIconsSquare {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
