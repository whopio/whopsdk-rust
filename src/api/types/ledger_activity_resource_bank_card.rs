pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LedgerActivityResourceBankCard {
    /// Card brand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    /// Card expiration month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,
    /// Card expiration year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,
    /// Last four digits of the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
}

impl LedgerActivityResourceBankCard {
    pub fn builder() -> LedgerActivityResourceBankCardBuilder {
        <LedgerActivityResourceBankCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerActivityResourceBankCardBuilder {
    brand: Option<String>,
    exp_month: Option<i64>,
    exp_year: Option<i64>,
    last4: Option<String>,
}

impl LedgerActivityResourceBankCardBuilder {
    pub fn brand(mut self, value: impl Into<String>) -> Self {
        self.brand = Some(value.into());
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

    /// Consumes the builder and constructs a [`LedgerActivityResourceBankCard`].
    pub fn build(self) -> Result<LedgerActivityResourceBankCard, BuildError> {
        Ok(LedgerActivityResourceBankCard {
            brand: self.brand,
            exp_month: self.exp_month,
            exp_year: self.exp_year,
            last4: self.last4,
        })
    }
}
