pub use crate::prelude::*;

/// The square tile (32x32).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentPaymentInstrumentIconsSquare {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentPaymentInstrumentIconsSquareDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentPaymentInstrumentIconsSquareLight,
}

impl PaymentPaymentInstrumentIconsSquare {
    pub fn builder() -> PaymentPaymentInstrumentIconsSquareBuilder {
        <PaymentPaymentInstrumentIconsSquareBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentPaymentInstrumentIconsSquareBuilder {
    dark: Option<PaymentPaymentInstrumentIconsSquareDark>,
    light: Option<PaymentPaymentInstrumentIconsSquareLight>,
}

impl PaymentPaymentInstrumentIconsSquareBuilder {
    pub fn dark(mut self, value: PaymentPaymentInstrumentIconsSquareDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentPaymentInstrumentIconsSquareLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentPaymentInstrumentIconsSquare`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentPaymentInstrumentIconsSquareBuilder::dark)
    /// - [`light`](PaymentPaymentInstrumentIconsSquareBuilder::light)
    pub fn build(self) -> Result<PaymentPaymentInstrumentIconsSquare, BuildError> {
        Ok(PaymentPaymentInstrumentIconsSquare {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
