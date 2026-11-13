pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaymentInstrumentCard {
    /// The network identifier (`visa`, `amex`, …), matching `card.networks` entries and saved card payment methods. Null when the vault did not record the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    /// The card's expiry month, 1 to 12. Null when the vault did not record it.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub exp_month: Option<f64>,
    /// The card's four-digit expiry year. Null when the vault did not record it.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub exp_year: Option<f64>,
    /// The issuer identification number, also called the BIN: the card's leading six or eight digits, which identify the issuing bank. Null when the processor did not report it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_identification_number: Option<String>,
    /// The card's last four digits, when captured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    /// The network transaction id this account uses for the card. It's this account's own id, from its first customer-present charge or verification on the card, when Whop has one. Otherwise it's an earlier network transaction id for this card, which can come from another account's transaction or be one brought with the card. Cite it as the prior transaction on a later merchant-initiated charge of this card at this account. Null until there is one. A webhook sent as a payment is first recorded can carry `null`, so retrieve the payment to read it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_transaction_id: Option<String>,
}

impl PaymentInstrumentCard {
    pub fn builder() -> PaymentInstrumentCardBuilder {
        <PaymentInstrumentCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentInstrumentCardBuilder {
    brand: Option<String>,
    exp_month: Option<f64>,
    exp_year: Option<f64>,
    issuer_identification_number: Option<String>,
    last4: Option<String>,
    network_transaction_id: Option<String>,
}

impl PaymentInstrumentCardBuilder {
    pub fn brand(mut self, value: impl Into<String>) -> Self {
        self.brand = Some(value.into());
        self
    }

    pub fn exp_month(mut self, value: f64) -> Self {
        self.exp_month = Some(value);
        self
    }

    pub fn exp_year(mut self, value: f64) -> Self {
        self.exp_year = Some(value);
        self
    }

    pub fn issuer_identification_number(mut self, value: impl Into<String>) -> Self {
        self.issuer_identification_number = Some(value.into());
        self
    }

    pub fn last4(mut self, value: impl Into<String>) -> Self {
        self.last4 = Some(value.into());
        self
    }

    pub fn network_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.network_transaction_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentInstrumentCard`].
    pub fn build(self) -> Result<PaymentInstrumentCard, BuildError> {
        Ok(PaymentInstrumentCard {
            brand: self.brand,
            exp_month: self.exp_month,
            exp_year: self.exp_year,
            issuer_identification_number: self.issuer_identification_number,
            last4: self.last4,
            network_transaction_id: self.network_transaction_id,
        })
    }
}
