pub use crate::prelude::*;

/// The card data associated with the payment method, if its a debit or credit card.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentPaymentMethodCard {
    /// The card network (e.g., visa, mastercard, amex). Null if the brand could not be determined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<CardBrands>,
    /// The two-digit expiration month of the card (1-12). Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,
    /// The two-digit expiration year of the card (e.g., 27 for 2027). Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,
    /// A stable identifier for the underlying card. Two payment methods with the same fingerprint are the same card. Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// The last four digits of the card number. Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
}

impl PaymentPaymentMethodCard {
    pub fn builder() -> PaymentPaymentMethodCardBuilder {
        <PaymentPaymentMethodCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentPaymentMethodCardBuilder {
    brand: Option<CardBrands>,
    exp_month: Option<i64>,
    exp_year: Option<i64>,
    fingerprint: Option<String>,
    last4: Option<String>,
}

impl PaymentPaymentMethodCardBuilder {
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

    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
        self
    }

    pub fn last4(mut self, value: impl Into<String>) -> Self {
        self.last4 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentPaymentMethodCard`].
    pub fn build(self) -> Result<PaymentPaymentMethodCard, BuildError> {
        Ok(PaymentPaymentMethodCard {
            brand: self.brand,
            exp_month: self.exp_month,
            exp_year: self.exp_year,
            fingerprint: self.fingerprint,
            last4: self.last4,
        })
    }
}
