pub use crate::prelude::*;

/// The square tile (32x32).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentListItemPaymentInstrumentIconsSquare {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentListItemPaymentInstrumentIconsSquareDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentListItemPaymentInstrumentIconsSquareLight,
}

impl PaymentListItemPaymentInstrumentIconsSquare {
    pub fn builder() -> PaymentListItemPaymentInstrumentIconsSquareBuilder {
        <PaymentListItemPaymentInstrumentIconsSquareBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentListItemPaymentInstrumentIconsSquareBuilder {
    dark: Option<PaymentListItemPaymentInstrumentIconsSquareDark>,
    light: Option<PaymentListItemPaymentInstrumentIconsSquareLight>,
}

impl PaymentListItemPaymentInstrumentIconsSquareBuilder {
    pub fn dark(mut self, value: PaymentListItemPaymentInstrumentIconsSquareDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentListItemPaymentInstrumentIconsSquareLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentListItemPaymentInstrumentIconsSquare`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentListItemPaymentInstrumentIconsSquareBuilder::dark)
    /// - [`light`](PaymentListItemPaymentInstrumentIconsSquareBuilder::light)
    pub fn build(self) -> Result<PaymentListItemPaymentInstrumentIconsSquare, BuildError> {
        Ok(PaymentListItemPaymentInstrumentIconsSquare {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
