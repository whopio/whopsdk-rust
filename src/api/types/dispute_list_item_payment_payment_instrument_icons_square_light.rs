pub use crate::prelude::*;

/// The colorway for light surfaces.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeListItemPaymentPaymentInstrumentIconsSquareLight {
    /// The vector file. Prefer this everywhere SVG renders.
    #[serde(default)]
    pub svg: String,
}

impl DisputeListItemPaymentPaymentInstrumentIconsSquareLight {
    pub fn builder() -> DisputeListItemPaymentPaymentInstrumentIconsSquareLightBuilder {
        <DisputeListItemPaymentPaymentInstrumentIconsSquareLightBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeListItemPaymentPaymentInstrumentIconsSquareLightBuilder {
    svg: Option<String>,
}

impl DisputeListItemPaymentPaymentInstrumentIconsSquareLightBuilder {
    pub fn svg(mut self, value: impl Into<String>) -> Self {
        self.svg = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisputeListItemPaymentPaymentInstrumentIconsSquareLight`].
    /// This method will fail if any of the following fields are not set:
    /// - [`svg`](DisputeListItemPaymentPaymentInstrumentIconsSquareLightBuilder::svg)
    pub fn build(
        self,
    ) -> Result<DisputeListItemPaymentPaymentInstrumentIconsSquareLight, BuildError> {
        Ok(DisputeListItemPaymentPaymentInstrumentIconsSquareLight {
            svg: self.svg.ok_or_else(|| BuildError::missing_field("svg"))?,
        })
    }
}
