pub use crate::prelude::*;

/// The colorway for dark surfaces.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareDark {
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

impl PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareDark {
    pub fn builder() -> PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareDarkBuilder {
        <PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareDarkBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareDarkBuilder {
    png1x: Option<String>,
    png2x: Option<String>,
    png4x: Option<String>,
    svg: Option<String>,
}

impl PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareDarkBuilder {
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

    /// Consumes the builder and constructs a [`PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareDark`].
    /// This method will fail if any of the following fields are not set:
    /// - [`png1x`](PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareDarkBuilder::png1x)
    /// - [`png2x`](PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareDarkBuilder::png2x)
    /// - [`png4x`](PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareDarkBuilder::png4x)
    /// - [`svg`](PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareDarkBuilder::svg)
    pub fn build(
        self,
    ) -> Result<PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareDark, BuildError> {
        Ok(
            PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareDark {
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
            },
        )
    }
}
