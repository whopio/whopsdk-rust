pub use crate::prelude::*;

/// The colorway for dark surfaces.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeLegacyPaymentPaymentInstrumentIconsSquareDark {
    /// The vector file. Prefer this everywhere SVG renders.
    #[serde(default)]
    pub svg: String,
}

impl DisputeLegacyPaymentPaymentInstrumentIconsSquareDark {
    pub fn builder() -> DisputeLegacyPaymentPaymentInstrumentIconsSquareDarkBuilder {
        <DisputeLegacyPaymentPaymentInstrumentIconsSquareDarkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeLegacyPaymentPaymentInstrumentIconsSquareDarkBuilder {
    svg: Option<String>,
}

impl DisputeLegacyPaymentPaymentInstrumentIconsSquareDarkBuilder {
    pub fn svg(mut self, value: impl Into<String>) -> Self {
        self.svg = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisputeLegacyPaymentPaymentInstrumentIconsSquareDark`].
    /// This method will fail if any of the following fields are not set:
    /// - [`svg`](DisputeLegacyPaymentPaymentInstrumentIconsSquareDarkBuilder::svg)
    pub fn build(self) -> Result<DisputeLegacyPaymentPaymentInstrumentIconsSquareDark, BuildError> {
        Ok(DisputeLegacyPaymentPaymentInstrumentIconsSquareDark {
            svg: self.svg.ok_or_else(|| BuildError::missing_field("svg"))?,
        })
    }
}
