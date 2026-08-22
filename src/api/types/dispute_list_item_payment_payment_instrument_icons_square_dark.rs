pub use crate::prelude::*;

/// The colorway for dark surfaces.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeListItemPaymentPaymentInstrumentIconsSquareDark {
    /// The vector file. Prefer this everywhere SVG renders.
    #[serde(default)]
    pub svg: String,
}

impl DisputeListItemPaymentPaymentInstrumentIconsSquareDark {
    pub fn builder() -> DisputeListItemPaymentPaymentInstrumentIconsSquareDarkBuilder {
        <DisputeListItemPaymentPaymentInstrumentIconsSquareDarkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeListItemPaymentPaymentInstrumentIconsSquareDarkBuilder {
    svg: Option<String>,
}

impl DisputeListItemPaymentPaymentInstrumentIconsSquareDarkBuilder {
    pub fn svg(mut self, value: impl Into<String>) -> Self {
        self.svg = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisputeListItemPaymentPaymentInstrumentIconsSquareDark`].
    /// This method will fail if any of the following fields are not set:
    /// - [`svg`](DisputeListItemPaymentPaymentInstrumentIconsSquareDarkBuilder::svg)
    pub fn build(
        self,
    ) -> Result<DisputeListItemPaymentPaymentInstrumentIconsSquareDark, BuildError> {
        Ok(DisputeListItemPaymentPaymentInstrumentIconsSquareDark {
            svg: self.svg.ok_or_else(|| BuildError::missing_field("svg"))?,
        })
    }
}
