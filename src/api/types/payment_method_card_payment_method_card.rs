pub use crate::prelude::*;

/// The card-specific details for this payment method, including brand, last four digits, and expiration.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodCardPaymentMethodCard {
    /// The card network (e.g., visa, mastercard, amex). Null if the brand could not be determined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<CardBrands>,
    /// The two-digit expiration month of the card (1-12). Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,
    /// The two-digit expiration year of the card (e.g., 27 for 2027). Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,
    /// Whether the card is past its expiration month. An expired card cannot take a new charge.
    #[serde(default)]
    pub expired: bool,
    /// A stable identifier for the underlying card. Two payment methods with the same fingerprint are the same card. Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// How the card is funded by the issuer. Null if the funding type could not be determined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<CardFundingTypes>,
    /// The last four digits of the card number. Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    /// Whether this card was verified with 3D Secure, either when it was saved or on a payment that used it.
    #[serde(default)]
    pub three_ds_verified: bool,
}

impl PaymentMethodCardPaymentMethodCard {
    pub fn builder() -> PaymentMethodCardPaymentMethodCardBuilder {
        <PaymentMethodCardPaymentMethodCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodCardPaymentMethodCardBuilder {
    brand: Option<CardBrands>,
    exp_month: Option<i64>,
    exp_year: Option<i64>,
    expired: Option<bool>,
    fingerprint: Option<String>,
    funding_type: Option<CardFundingTypes>,
    last4: Option<String>,
    three_ds_verified: Option<bool>,
}

impl PaymentMethodCardPaymentMethodCardBuilder {
    pub fn brand(mut self, value: CardBrands) -> Self {
        self.brand = Some(value);
        self
    }

    pub fn exp_month(mut self, value: i64) -> Self {
        self.exp_month = Some(value);
        self
    }

    pub fn exp_year(mut self, value: i64) -> Self {
        self.exp_year = Some(value);
        self
    }

    pub fn expired(mut self, value: bool) -> Self {
        self.expired = Some(value);
        self
    }

    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
        self
    }

    pub fn funding_type(mut self, value: CardFundingTypes) -> Self {
        self.funding_type = Some(value);
        self
    }

    pub fn last4(mut self, value: impl Into<String>) -> Self {
        self.last4 = Some(value.into());
        self
    }

    pub fn three_ds_verified(mut self, value: bool) -> Self {
        self.three_ds_verified = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodCardPaymentMethodCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expired`](PaymentMethodCardPaymentMethodCardBuilder::expired)
    /// - [`three_ds_verified`](PaymentMethodCardPaymentMethodCardBuilder::three_ds_verified)
    pub fn build(self) -> Result<PaymentMethodCardPaymentMethodCard, BuildError> {
        Ok(PaymentMethodCardPaymentMethodCard {
            brand: self.brand,
            exp_month: self.exp_month,
            exp_year: self.exp_year,
            expired: self
                .expired
                .ok_or_else(|| BuildError::missing_field("expired"))?,
            fingerprint: self.fingerprint,
            funding_type: self.funding_type,
            last4: self.last4,
            three_ds_verified: self
                .three_ds_verified
                .ok_or_else(|| BuildError::missing_field("three_ds_verified"))?,
        })
    }
}
