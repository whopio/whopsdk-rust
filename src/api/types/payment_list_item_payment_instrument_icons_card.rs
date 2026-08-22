pub use crate::prelude::*;

/// The credit-card-proportioned tile (48x30).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentListItemPaymentInstrumentIconsCard {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentListItemPaymentInstrumentIconsCardDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentListItemPaymentInstrumentIconsCardLight,
}

impl PaymentListItemPaymentInstrumentIconsCard {
    pub fn builder() -> PaymentListItemPaymentInstrumentIconsCardBuilder {
        <PaymentListItemPaymentInstrumentIconsCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentListItemPaymentInstrumentIconsCardBuilder {
    dark: Option<PaymentListItemPaymentInstrumentIconsCardDark>,
    light: Option<PaymentListItemPaymentInstrumentIconsCardLight>,
}

impl PaymentListItemPaymentInstrumentIconsCardBuilder {
    pub fn dark(mut self, value: PaymentListItemPaymentInstrumentIconsCardDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentListItemPaymentInstrumentIconsCardLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentListItemPaymentInstrumentIconsCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentListItemPaymentInstrumentIconsCardBuilder::dark)
    /// - [`light`](PaymentListItemPaymentInstrumentIconsCardBuilder::light)
    pub fn build(self) -> Result<PaymentListItemPaymentInstrumentIconsCard, BuildError> {
        Ok(PaymentListItemPaymentInstrumentIconsCard {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
