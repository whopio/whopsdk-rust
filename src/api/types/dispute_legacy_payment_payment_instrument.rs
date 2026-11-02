pub use crate::prelude::*;

/// The instrument this payment was made with, shaped for display: the method type, a buyer-facing name, the standard icon set, and the card facts when it was a card. Null when the receipt names no payment method.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeLegacyPaymentPaymentInstrument {
    /// Buyer-facing instrument name — "Visa •••• 4242" when the card surfaced, else the method's own name ("Klarna").
    #[serde(default)]
    pub display_name: String,
    /// The standard icon set: square and card shapes, each in light and dark colorways.
    #[serde(default)]
    pub icons: DisputeLegacyPaymentPaymentInstrumentIcons,
    /// Installment methods only: how many payments the charge splits into. Data, not copy — compose and translate the label client-side.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installment_count: Option<i64>,
    /// The payment method type identifier, e.g. `card`, `klarna`, `apple_pay`.
    #[serde(default)]
    pub payment_method_type: String,
}

impl DisputeLegacyPaymentPaymentInstrument {
    pub fn builder() -> DisputeLegacyPaymentPaymentInstrumentBuilder {
        <DisputeLegacyPaymentPaymentInstrumentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeLegacyPaymentPaymentInstrumentBuilder {
    display_name: Option<String>,
    icons: Option<DisputeLegacyPaymentPaymentInstrumentIcons>,
    installment_count: Option<i64>,
    payment_method_type: Option<String>,
}

impl DisputeLegacyPaymentPaymentInstrumentBuilder {
    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn icons(mut self, value: DisputeLegacyPaymentPaymentInstrumentIcons) -> Self {
        self.icons = Some(value);
        self
    }

    pub fn installment_count(mut self, value: i64) -> Self {
        self.installment_count = Some(value);
        self
    }

    pub fn payment_method_type(mut self, value: impl Into<String>) -> Self {
        self.payment_method_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisputeLegacyPaymentPaymentInstrument`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display_name`](DisputeLegacyPaymentPaymentInstrumentBuilder::display_name)
    /// - [`icons`](DisputeLegacyPaymentPaymentInstrumentBuilder::icons)
    /// - [`payment_method_type`](DisputeLegacyPaymentPaymentInstrumentBuilder::payment_method_type)
    pub fn build(self) -> Result<DisputeLegacyPaymentPaymentInstrument, BuildError> {
        Ok(DisputeLegacyPaymentPaymentInstrument {
            display_name: self
                .display_name
                .ok_or_else(|| BuildError::missing_field("display_name"))?,
            icons: self
                .icons
                .ok_or_else(|| BuildError::missing_field("icons"))?,
            installment_count: self.installment_count,
            payment_method_type: self
                .payment_method_type
                .ok_or_else(|| BuildError::missing_field("payment_method_type"))?,
        })
    }
}
