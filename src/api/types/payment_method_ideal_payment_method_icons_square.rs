pub use crate::prelude::*;

/// The square tile (32x32).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodIdealPaymentMethodIconsSquare {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodIdealPaymentMethodIconsSquareDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodIdealPaymentMethodIconsSquareLight,
}

impl PaymentMethodIdealPaymentMethodIconsSquare {
    pub fn builder() -> PaymentMethodIdealPaymentMethodIconsSquareBuilder {
        <PaymentMethodIdealPaymentMethodIconsSquareBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodIdealPaymentMethodIconsSquareBuilder {
    dark: Option<PaymentMethodIdealPaymentMethodIconsSquareDark>,
    light: Option<PaymentMethodIdealPaymentMethodIconsSquareLight>,
}

impl PaymentMethodIdealPaymentMethodIconsSquareBuilder {
    pub fn dark(mut self, value: PaymentMethodIdealPaymentMethodIconsSquareDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodIdealPaymentMethodIconsSquareLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodIdealPaymentMethodIconsSquare`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodIdealPaymentMethodIconsSquareBuilder::dark)
    /// - [`light`](PaymentMethodIdealPaymentMethodIconsSquareBuilder::light)
    pub fn build(self) -> Result<PaymentMethodIdealPaymentMethodIconsSquare, BuildError> {
        Ok(PaymentMethodIdealPaymentMethodIconsSquare {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
