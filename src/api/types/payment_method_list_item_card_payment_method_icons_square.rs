pub use crate::prelude::*;

/// The square tile (32x32).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemCardPaymentMethodIconsSquare {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodListItemCardPaymentMethodIconsSquareDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodListItemCardPaymentMethodIconsSquareLight,
}

impl PaymentMethodListItemCardPaymentMethodIconsSquare {
    pub fn builder() -> PaymentMethodListItemCardPaymentMethodIconsSquareBuilder {
        <PaymentMethodListItemCardPaymentMethodIconsSquareBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemCardPaymentMethodIconsSquareBuilder {
    dark: Option<PaymentMethodListItemCardPaymentMethodIconsSquareDark>,
    light: Option<PaymentMethodListItemCardPaymentMethodIconsSquareLight>,
}

impl PaymentMethodListItemCardPaymentMethodIconsSquareBuilder {
    pub fn dark(mut self, value: PaymentMethodListItemCardPaymentMethodIconsSquareDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodListItemCardPaymentMethodIconsSquareLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemCardPaymentMethodIconsSquare`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodListItemCardPaymentMethodIconsSquareBuilder::dark)
    /// - [`light`](PaymentMethodListItemCardPaymentMethodIconsSquareBuilder::light)
    pub fn build(self) -> Result<PaymentMethodListItemCardPaymentMethodIconsSquare, BuildError> {
        Ok(PaymentMethodListItemCardPaymentMethodIconsSquare {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
