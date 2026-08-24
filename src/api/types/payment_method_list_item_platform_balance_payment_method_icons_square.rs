pub use crate::prelude::*;

/// The square tile (32x32).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemPlatformBalancePaymentMethodIconsSquare {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodListItemPlatformBalancePaymentMethodIconsSquareDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodListItemPlatformBalancePaymentMethodIconsSquareLight,
}

impl PaymentMethodListItemPlatformBalancePaymentMethodIconsSquare {
    pub fn builder() -> PaymentMethodListItemPlatformBalancePaymentMethodIconsSquareBuilder {
        <PaymentMethodListItemPlatformBalancePaymentMethodIconsSquareBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemPlatformBalancePaymentMethodIconsSquareBuilder {
    dark: Option<PaymentMethodListItemPlatformBalancePaymentMethodIconsSquareDark>,
    light: Option<PaymentMethodListItemPlatformBalancePaymentMethodIconsSquareLight>,
}

impl PaymentMethodListItemPlatformBalancePaymentMethodIconsSquareBuilder {
    pub fn dark(
        mut self,
        value: PaymentMethodListItemPlatformBalancePaymentMethodIconsSquareDark,
    ) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(
        mut self,
        value: PaymentMethodListItemPlatformBalancePaymentMethodIconsSquareLight,
    ) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemPlatformBalancePaymentMethodIconsSquare`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodListItemPlatformBalancePaymentMethodIconsSquareBuilder::dark)
    /// - [`light`](PaymentMethodListItemPlatformBalancePaymentMethodIconsSquareBuilder::light)
    pub fn build(
        self,
    ) -> Result<PaymentMethodListItemPlatformBalancePaymentMethodIconsSquare, BuildError> {
        Ok(
            PaymentMethodListItemPlatformBalancePaymentMethodIconsSquare {
                dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
                light: self
                    .light
                    .ok_or_else(|| BuildError::missing_field("light"))?,
            },
        )
    }
}
