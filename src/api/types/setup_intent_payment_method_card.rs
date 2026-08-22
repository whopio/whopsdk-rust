pub use crate::prelude::*;

/// The card data associated with the payment method, if its a debit or credit card.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetupIntentPaymentMethodCard {
    /// The card network (e.g., visa, mastercard, amex). Null if the brand could not be determined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<CardBrands>,
    /// The two-digit expiration month of the card (1-12). Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,
    /// The two-digit expiration year of the card (e.g., 27 for 2027). Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,
    /// The last four digits of the card number. Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
}

impl SetupIntentPaymentMethodCard {
    pub fn builder() -> SetupIntentPaymentMethodCardBuilder {
        <SetupIntentPaymentMethodCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetupIntentPaymentMethodCardBuilder {
    brand: Option<CardBrands>,
    exp_month: Option<i64>,
    exp_year: Option<i64>,
    last4: Option<String>,
}

impl SetupIntentPaymentMethodCardBuilder {
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

    pub fn last4(mut self, value: impl Into<String>) -> Self {
        self.last4 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetupIntentPaymentMethodCard`].
    pub fn build(self) -> Result<SetupIntentPaymentMethodCard, BuildError> {
        Ok(SetupIntentPaymentMethodCard {
            brand: self.brand,
            exp_month: self.exp_month,
            exp_year: self.exp_year,
            last4: self.last4,
        })
    }
}
