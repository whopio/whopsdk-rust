pub use crate::prelude::*;

/// The standard icon set: square and card shapes, each in light and dark colorways.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeLegacyPaymentPaymentInstrumentIcons {
    /// The square tile (32x32).
    #[serde(default)]
    pub square: DisputeLegacyPaymentPaymentInstrumentIconsSquare,
}

impl DisputeLegacyPaymentPaymentInstrumentIcons {
    pub fn builder() -> DisputeLegacyPaymentPaymentInstrumentIconsBuilder {
        <DisputeLegacyPaymentPaymentInstrumentIconsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeLegacyPaymentPaymentInstrumentIconsBuilder {
    square: Option<DisputeLegacyPaymentPaymentInstrumentIconsSquare>,
}

impl DisputeLegacyPaymentPaymentInstrumentIconsBuilder {
    pub fn square(mut self, value: DisputeLegacyPaymentPaymentInstrumentIconsSquare) -> Self {
        self.square = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DisputeLegacyPaymentPaymentInstrumentIcons`].
    /// This method will fail if any of the following fields are not set:
    /// - [`square`](DisputeLegacyPaymentPaymentInstrumentIconsBuilder::square)
    pub fn build(self) -> Result<DisputeLegacyPaymentPaymentInstrumentIcons, BuildError> {
        Ok(DisputeLegacyPaymentPaymentInstrumentIcons {
            square: self
                .square
                .ok_or_else(|| BuildError::missing_field("square"))?,
        })
    }
}
