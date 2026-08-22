pub use crate::prelude::*;

/// The colorway for light surfaces.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodCardPaymentMethodIconsSquareLight {
    /// Raster fallback at the shape's native size.
    #[serde(rename = "png_1x")]
    #[serde(default)]
    pub png1x: String,
    /// Raster fallback at double density.
    #[serde(rename = "png_2x")]
    #[serde(default)]
    pub png2x: String,
    /// Raster fallback at quadruple density.
    #[serde(rename = "png_4x")]
    #[serde(default)]
    pub png4x: String,
    /// The vector file. Prefer this everywhere SVG renders.
    #[serde(default)]
    pub svg: String,
}

impl PaymentMethodCardPaymentMethodIconsSquareLight {
    pub fn builder() -> PaymentMethodCardPaymentMethodIconsSquareLightBuilder {
        <PaymentMethodCardPaymentMethodIconsSquareLightBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodCardPaymentMethodIconsSquareLightBuilder {
    png1x: Option<String>,
    png2x: Option<String>,
    png4x: Option<String>,
    svg: Option<String>,
}

impl PaymentMethodCardPaymentMethodIconsSquareLightBuilder {
    pub fn png1x(mut self, value: impl Into<String>) -> Self {
        self.png1x = Some(value.into());
        self
    }

    pub fn png2x(mut self, value: impl Into<String>) -> Self {
        self.png2x = Some(value.into());
        self
    }

    pub fn png4x(mut self, value: impl Into<String>) -> Self {
        self.png4x = Some(value.into());
        self
    }

    pub fn svg(mut self, value: impl Into<String>) -> Self {
        self.svg = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodCardPaymentMethodIconsSquareLight`].
    /// This method will fail if any of the following fields are not set:
    /// - [`png1x`](PaymentMethodCardPaymentMethodIconsSquareLightBuilder::png1x)
    /// - [`png2x`](PaymentMethodCardPaymentMethodIconsSquareLightBuilder::png2x)
    /// - [`png4x`](PaymentMethodCardPaymentMethodIconsSquareLightBuilder::png4x)
    /// - [`svg`](PaymentMethodCardPaymentMethodIconsSquareLightBuilder::svg)
    pub fn build(self) -> Result<PaymentMethodCardPaymentMethodIconsSquareLight, BuildError> {
        Ok(PaymentMethodCardPaymentMethodIconsSquareLight {
            png1x: self
                .png1x
                .ok_or_else(|| BuildError::missing_field("png1x"))?,
            png2x: self
                .png2x
                .ok_or_else(|| BuildError::missing_field("png2x"))?,
            png4x: self
                .png4x
                .ok_or_else(|| BuildError::missing_field("png4x"))?,
            svg: self.svg.ok_or_else(|| BuildError::missing_field("svg"))?,
        })
    }
}
