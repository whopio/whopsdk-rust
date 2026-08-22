pub use crate::prelude::*;

/// The standard icon set: square and card shapes, each in light and dark colorways.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeListItemPaymentPaymentInstrumentIcons {
    /// The square tile (32x32).
    #[serde(default)]
    pub square: DisputeListItemPaymentPaymentInstrumentIconsSquare,
}

impl DisputeListItemPaymentPaymentInstrumentIcons {
    pub fn builder() -> DisputeListItemPaymentPaymentInstrumentIconsBuilder {
        <DisputeListItemPaymentPaymentInstrumentIconsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeListItemPaymentPaymentInstrumentIconsBuilder {
    square: Option<DisputeListItemPaymentPaymentInstrumentIconsSquare>,
}

impl DisputeListItemPaymentPaymentInstrumentIconsBuilder {
    pub fn square(mut self, value: DisputeListItemPaymentPaymentInstrumentIconsSquare) -> Self {
        self.square = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DisputeListItemPaymentPaymentInstrumentIcons`].
    /// This method will fail if any of the following fields are not set:
    /// - [`square`](DisputeListItemPaymentPaymentInstrumentIconsBuilder::square)
    pub fn build(self) -> Result<DisputeListItemPaymentPaymentInstrumentIcons, BuildError> {
        Ok(DisputeListItemPaymentPaymentInstrumentIcons {
            square: self
                .square
                .ok_or_else(|| BuildError::missing_field("square"))?,
        })
    }
}
