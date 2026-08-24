pub use crate::prelude::*;

/// The credit-card-proportioned tile (48x30).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemPlatformBalancePaymentMethodIconsCard {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodListItemPlatformBalancePaymentMethodIconsCardDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodListItemPlatformBalancePaymentMethodIconsCardLight,
}

impl PaymentMethodListItemPlatformBalancePaymentMethodIconsCard {
    pub fn builder() -> PaymentMethodListItemPlatformBalancePaymentMethodIconsCardBuilder {
        <PaymentMethodListItemPlatformBalancePaymentMethodIconsCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemPlatformBalancePaymentMethodIconsCardBuilder {
    dark: Option<PaymentMethodListItemPlatformBalancePaymentMethodIconsCardDark>,
    light: Option<PaymentMethodListItemPlatformBalancePaymentMethodIconsCardLight>,
}

impl PaymentMethodListItemPlatformBalancePaymentMethodIconsCardBuilder {
    pub fn dark(
        mut self,
        value: PaymentMethodListItemPlatformBalancePaymentMethodIconsCardDark,
    ) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(
        mut self,
        value: PaymentMethodListItemPlatformBalancePaymentMethodIconsCardLight,
    ) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemPlatformBalancePaymentMethodIconsCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodListItemPlatformBalancePaymentMethodIconsCardBuilder::dark)
    /// - [`light`](PaymentMethodListItemPlatformBalancePaymentMethodIconsCardBuilder::light)
    pub fn build(
        self,
    ) -> Result<PaymentMethodListItemPlatformBalancePaymentMethodIconsCard, BuildError> {
        Ok(PaymentMethodListItemPlatformBalancePaymentMethodIconsCard {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
