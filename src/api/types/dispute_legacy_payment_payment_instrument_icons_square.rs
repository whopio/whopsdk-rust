pub use crate::prelude::*;

/// The square tile (32x32).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeLegacyPaymentPaymentInstrumentIconsSquare {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: DisputeLegacyPaymentPaymentInstrumentIconsSquareDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: DisputeLegacyPaymentPaymentInstrumentIconsSquareLight,
}

impl DisputeLegacyPaymentPaymentInstrumentIconsSquare {
    pub fn builder() -> DisputeLegacyPaymentPaymentInstrumentIconsSquareBuilder {
        <DisputeLegacyPaymentPaymentInstrumentIconsSquareBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeLegacyPaymentPaymentInstrumentIconsSquareBuilder {
    dark: Option<DisputeLegacyPaymentPaymentInstrumentIconsSquareDark>,
    light: Option<DisputeLegacyPaymentPaymentInstrumentIconsSquareLight>,
}

impl DisputeLegacyPaymentPaymentInstrumentIconsSquareBuilder {
    pub fn dark(mut self, value: DisputeLegacyPaymentPaymentInstrumentIconsSquareDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: DisputeLegacyPaymentPaymentInstrumentIconsSquareLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DisputeLegacyPaymentPaymentInstrumentIconsSquare`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](DisputeLegacyPaymentPaymentInstrumentIconsSquareBuilder::dark)
    /// - [`light`](DisputeLegacyPaymentPaymentInstrumentIconsSquareBuilder::light)
    pub fn build(self) -> Result<DisputeLegacyPaymentPaymentInstrumentIconsSquare, BuildError> {
        Ok(DisputeLegacyPaymentPaymentInstrumentIconsSquare {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
