pub use crate::prelude::*;

/// Currency for this ledger activity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LedgerActivityCurrency {
    /// Currency code.
    #[serde(default)]
    pub code: String,
    /// Precision factor for the currency, for example `100000000` for USD.
    #[serde(default)]
    pub precision: String,
}

impl LedgerActivityCurrency {
    pub fn builder() -> LedgerActivityCurrencyBuilder {
        <LedgerActivityCurrencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerActivityCurrencyBuilder {
    code: Option<String>,
    precision: Option<String>,
}

impl LedgerActivityCurrencyBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn precision(mut self, value: impl Into<String>) -> Self {
        self.precision = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LedgerActivityCurrency`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](LedgerActivityCurrencyBuilder::code)
    /// - [`precision`](LedgerActivityCurrencyBuilder::precision)
    pub fn build(self) -> Result<LedgerActivityCurrency, BuildError> {
        Ok(LedgerActivityCurrency {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            precision: self
                .precision
                .ok_or_else(|| BuildError::missing_field("precision"))?,
        })
    }
}
