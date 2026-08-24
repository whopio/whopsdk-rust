pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Money {
    /// The amount in major units, as an exact decimal string — `"10.00"` is ten dollars. A string so no float rounds it in transit.
    #[serde(default)]
    pub amount: String,
    /// Three-letter ISO 4217 currency code, lowercase.
    #[serde(default)]
    pub currency: String,
    /// How many decimal places the amount CARRIES — the precision the charge itself runs at.
    #[serde(default)]
    pub decimals: i64,
    /// How many decimal places to SHOW. Usually equal to `decimals`, and deliberately not always: COP is charged in centavos but written in whole pesos, so it is `2` and `0`. Format the number in your own locale using this.
    #[serde(default)]
    pub display_decimals: i64,
}

impl Money {
    pub fn builder() -> MoneyBuilder {
        <MoneyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MoneyBuilder {
    amount: Option<String>,
    currency: Option<String>,
    decimals: Option<i64>,
    display_decimals: Option<i64>,
}

impl MoneyBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn decimals(mut self, value: i64) -> Self {
        self.decimals = Some(value);
        self
    }

    pub fn display_decimals(mut self, value: i64) -> Self {
        self.display_decimals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Money`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](MoneyBuilder::amount)
    /// - [`currency`](MoneyBuilder::currency)
    /// - [`decimals`](MoneyBuilder::decimals)
    /// - [`display_decimals`](MoneyBuilder::display_decimals)
    pub fn build(self) -> Result<Money, BuildError> {
        Ok(Money {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            decimals: self
                .decimals
                .ok_or_else(|| BuildError::missing_field("decimals"))?,
            display_decimals: self
                .display_decimals
                .ok_or_else(|| BuildError::missing_field("display_decimals"))?,
        })
    }
}
