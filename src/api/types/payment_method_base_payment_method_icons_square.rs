pub use crate::prelude::*;

/// The square tile (32x32).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodBasePaymentMethodIconsSquare {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodBasePaymentMethodIconsSquareDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodBasePaymentMethodIconsSquareLight,
}

impl PaymentMethodBasePaymentMethodIconsSquare {
    pub fn builder() -> PaymentMethodBasePaymentMethodIconsSquareBuilder {
        <PaymentMethodBasePaymentMethodIconsSquareBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodBasePaymentMethodIconsSquareBuilder {
    dark: Option<PaymentMethodBasePaymentMethodIconsSquareDark>,
    light: Option<PaymentMethodBasePaymentMethodIconsSquareLight>,
}

impl PaymentMethodBasePaymentMethodIconsSquareBuilder {
    pub fn dark(mut self, value: PaymentMethodBasePaymentMethodIconsSquareDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodBasePaymentMethodIconsSquareLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodBasePaymentMethodIconsSquare`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodBasePaymentMethodIconsSquareBuilder::dark)
    /// - [`light`](PaymentMethodBasePaymentMethodIconsSquareBuilder::light)
    pub fn build(self) -> Result<PaymentMethodBasePaymentMethodIconsSquare, BuildError> {
        Ok(PaymentMethodBasePaymentMethodIconsSquare {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
