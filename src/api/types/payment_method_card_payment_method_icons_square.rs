pub use crate::prelude::*;

/// The square tile (32x32).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodCardPaymentMethodIconsSquare {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodCardPaymentMethodIconsSquareDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodCardPaymentMethodIconsSquareLight,
}

impl PaymentMethodCardPaymentMethodIconsSquare {
    pub fn builder() -> PaymentMethodCardPaymentMethodIconsSquareBuilder {
        <PaymentMethodCardPaymentMethodIconsSquareBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodCardPaymentMethodIconsSquareBuilder {
    dark: Option<PaymentMethodCardPaymentMethodIconsSquareDark>,
    light: Option<PaymentMethodCardPaymentMethodIconsSquareLight>,
}

impl PaymentMethodCardPaymentMethodIconsSquareBuilder {
    pub fn dark(mut self, value: PaymentMethodCardPaymentMethodIconsSquareDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodCardPaymentMethodIconsSquareLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodCardPaymentMethodIconsSquare`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodCardPaymentMethodIconsSquareBuilder::dark)
    /// - [`light`](PaymentMethodCardPaymentMethodIconsSquareBuilder::light)
    pub fn build(self) -> Result<PaymentMethodCardPaymentMethodIconsSquare, BuildError> {
        Ok(PaymentMethodCardPaymentMethodIconsSquare {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
