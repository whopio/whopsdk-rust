pub use crate::prelude::*;

/// The colorway for light surfaces.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeLegacyPaymentPaymentInstrumentIconsSquareLight {
    /// The vector file. Prefer this everywhere SVG renders.
    #[serde(default)]
    pub svg: String,
}

impl DisputeLegacyPaymentPaymentInstrumentIconsSquareLight {
    pub fn builder() -> DisputeLegacyPaymentPaymentInstrumentIconsSquareLightBuilder {
        <DisputeLegacyPaymentPaymentInstrumentIconsSquareLightBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeLegacyPaymentPaymentInstrumentIconsSquareLightBuilder {
    svg: Option<String>,
}

impl DisputeLegacyPaymentPaymentInstrumentIconsSquareLightBuilder {
    pub fn svg(mut self, value: impl Into<String>) -> Self {
        self.svg = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisputeLegacyPaymentPaymentInstrumentIconsSquareLight`].
    /// This method will fail if any of the following fields are not set:
    /// - [`svg`](DisputeLegacyPaymentPaymentInstrumentIconsSquareLightBuilder::svg)
    pub fn build(
        self,
    ) -> Result<DisputeLegacyPaymentPaymentInstrumentIconsSquareLight, BuildError> {
        Ok(DisputeLegacyPaymentPaymentInstrumentIconsSquareLight {
            svg: self.svg.ok_or_else(|| BuildError::missing_field("svg"))?,
        })
    }
}
