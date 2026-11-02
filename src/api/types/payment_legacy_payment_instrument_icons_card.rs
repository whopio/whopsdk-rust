pub use crate::prelude::*;

/// The credit-card-proportioned tile (48x30).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentLegacyPaymentInstrumentIconsCard {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentLegacyPaymentInstrumentIconsCardDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentLegacyPaymentInstrumentIconsCardLight,
}

impl PaymentLegacyPaymentInstrumentIconsCard {
    pub fn builder() -> PaymentLegacyPaymentInstrumentIconsCardBuilder {
        <PaymentLegacyPaymentInstrumentIconsCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentLegacyPaymentInstrumentIconsCardBuilder {
    dark: Option<PaymentLegacyPaymentInstrumentIconsCardDark>,
    light: Option<PaymentLegacyPaymentInstrumentIconsCardLight>,
}

impl PaymentLegacyPaymentInstrumentIconsCardBuilder {
    pub fn dark(mut self, value: PaymentLegacyPaymentInstrumentIconsCardDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentLegacyPaymentInstrumentIconsCardLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentLegacyPaymentInstrumentIconsCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentLegacyPaymentInstrumentIconsCardBuilder::dark)
    /// - [`light`](PaymentLegacyPaymentInstrumentIconsCardBuilder::light)
    pub fn build(self) -> Result<PaymentLegacyPaymentInstrumentIconsCard, BuildError> {
        Ok(PaymentLegacyPaymentInstrumentIconsCard {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
