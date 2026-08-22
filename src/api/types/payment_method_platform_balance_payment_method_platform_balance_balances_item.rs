pub use crate::prelude::*;

/// An amount of money. Never a bare number, because a bare number cannot answer the two questions a client has to answer to render it: what currency is this, and how many digits do I write? The second is stated twice rather than derived, because the digits the amount CARRIES and the digits to SHOW differ in COP — charged in centavos, written in whole pesos. Formatting is deliberately left to the caller: the number belongs in the buyer's locale, and this API does not know it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentMethodPlatformBalancePaymentMethodPlatformBalanceBalancesItem {
    /// The amount in major units, as an exact decimal string — `"10.00"` is ten dollars. A string so no float rounds it in transit.
    #[serde(default)]
    pub amount: String,
    /// Three-letter ISO 4217 currency code, lowercase.
    pub currency: Currencies,
    /// How many decimal places the amount CARRIES — the precision the charge itself runs at.
    #[serde(default)]
    pub decimals: i64,
    /// How many decimal places to SHOW. Usually equal to `decimals`, and deliberately not always: COP is charged in centavos but written in whole pesos, so it is `2` and `0`. Format the number in your own locale using this.
    #[serde(default)]
    pub display_decimals: i64,
}

impl PaymentMethodPlatformBalancePaymentMethodPlatformBalanceBalancesItem {
    pub fn builder() -> PaymentMethodPlatformBalancePaymentMethodPlatformBalanceBalancesItemBuilder
    {
        <PaymentMethodPlatformBalancePaymentMethodPlatformBalanceBalancesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodPlatformBalancePaymentMethodPlatformBalanceBalancesItemBuilder {
    amount: Option<String>,
    currency: Option<Currencies>,
    decimals: Option<i64>,
    display_decimals: Option<i64>,
}

impl PaymentMethodPlatformBalancePaymentMethodPlatformBalanceBalancesItemBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn currency(mut self, value: Currencies) -> Self {
        self.currency = Some(value);
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

    /// Consumes the builder and constructs a [`PaymentMethodPlatformBalancePaymentMethodPlatformBalanceBalancesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](PaymentMethodPlatformBalancePaymentMethodPlatformBalanceBalancesItemBuilder::amount)
    /// - [`currency`](PaymentMethodPlatformBalancePaymentMethodPlatformBalanceBalancesItemBuilder::currency)
    /// - [`decimals`](PaymentMethodPlatformBalancePaymentMethodPlatformBalanceBalancesItemBuilder::decimals)
    /// - [`display_decimals`](PaymentMethodPlatformBalancePaymentMethodPlatformBalanceBalancesItemBuilder::display_decimals)
    pub fn build(
        self,
    ) -> Result<PaymentMethodPlatformBalancePaymentMethodPlatformBalanceBalancesItem, BuildError>
    {
        Ok(
            PaymentMethodPlatformBalancePaymentMethodPlatformBalanceBalancesItem {
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
            },
        )
    }
}
