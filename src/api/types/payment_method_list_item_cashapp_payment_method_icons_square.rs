pub use crate::prelude::*;

/// The square tile (32x32).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemCashappPaymentMethodIconsSquare {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodListItemCashappPaymentMethodIconsSquareDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodListItemCashappPaymentMethodIconsSquareLight,
}

impl PaymentMethodListItemCashappPaymentMethodIconsSquare {
    pub fn builder() -> PaymentMethodListItemCashappPaymentMethodIconsSquareBuilder {
        <PaymentMethodListItemCashappPaymentMethodIconsSquareBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemCashappPaymentMethodIconsSquareBuilder {
    dark: Option<PaymentMethodListItemCashappPaymentMethodIconsSquareDark>,
    light: Option<PaymentMethodListItemCashappPaymentMethodIconsSquareLight>,
}

impl PaymentMethodListItemCashappPaymentMethodIconsSquareBuilder {
    pub fn dark(mut self, value: PaymentMethodListItemCashappPaymentMethodIconsSquareDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(
        mut self,
        value: PaymentMethodListItemCashappPaymentMethodIconsSquareLight,
    ) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemCashappPaymentMethodIconsSquare`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodListItemCashappPaymentMethodIconsSquareBuilder::dark)
    /// - [`light`](PaymentMethodListItemCashappPaymentMethodIconsSquareBuilder::light)
    pub fn build(self) -> Result<PaymentMethodListItemCashappPaymentMethodIconsSquare, BuildError> {
        Ok(PaymentMethodListItemCashappPaymentMethodIconsSquare {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
