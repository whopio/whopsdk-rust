pub use crate::prelude::*;

/// The square tile (32x32).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeListItemPaymentPaymentInstrumentIconsSquare {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: DisputeListItemPaymentPaymentInstrumentIconsSquareDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: DisputeListItemPaymentPaymentInstrumentIconsSquareLight,
}

impl DisputeListItemPaymentPaymentInstrumentIconsSquare {
    pub fn builder() -> DisputeListItemPaymentPaymentInstrumentIconsSquareBuilder {
        <DisputeListItemPaymentPaymentInstrumentIconsSquareBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeListItemPaymentPaymentInstrumentIconsSquareBuilder {
    dark: Option<DisputeListItemPaymentPaymentInstrumentIconsSquareDark>,
    light: Option<DisputeListItemPaymentPaymentInstrumentIconsSquareLight>,
}

impl DisputeListItemPaymentPaymentInstrumentIconsSquareBuilder {
    pub fn dark(mut self, value: DisputeListItemPaymentPaymentInstrumentIconsSquareDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: DisputeListItemPaymentPaymentInstrumentIconsSquareLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DisputeListItemPaymentPaymentInstrumentIconsSquare`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](DisputeListItemPaymentPaymentInstrumentIconsSquareBuilder::dark)
    /// - [`light`](DisputeListItemPaymentPaymentInstrumentIconsSquareBuilder::light)
    pub fn build(self) -> Result<DisputeListItemPaymentPaymentInstrumentIconsSquare, BuildError> {
        Ok(DisputeListItemPaymentPaymentInstrumentIconsSquare {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
