pub use crate::prelude::*;

/// The square tile (32x32).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentLegacyPaymentInstrumentIconsSquare {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentLegacyPaymentInstrumentIconsSquareDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentLegacyPaymentInstrumentIconsSquareLight,
}

impl PaymentLegacyPaymentInstrumentIconsSquare {
    pub fn builder() -> PaymentLegacyPaymentInstrumentIconsSquareBuilder {
        <PaymentLegacyPaymentInstrumentIconsSquareBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentLegacyPaymentInstrumentIconsSquareBuilder {
    dark: Option<PaymentLegacyPaymentInstrumentIconsSquareDark>,
    light: Option<PaymentLegacyPaymentInstrumentIconsSquareLight>,
}

impl PaymentLegacyPaymentInstrumentIconsSquareBuilder {
    pub fn dark(mut self, value: PaymentLegacyPaymentInstrumentIconsSquareDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentLegacyPaymentInstrumentIconsSquareLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentLegacyPaymentInstrumentIconsSquare`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentLegacyPaymentInstrumentIconsSquareBuilder::dark)
    /// - [`light`](PaymentLegacyPaymentInstrumentIconsSquareBuilder::light)
    pub fn build(self) -> Result<PaymentLegacyPaymentInstrumentIconsSquare, BuildError> {
        Ok(PaymentLegacyPaymentInstrumentIconsSquare {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
