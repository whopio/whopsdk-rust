pub use crate::prelude::*;

/// Bank deposit details. Only present when bank deposits are active for the destination account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDepositsResponseMethodsBank {
    /// Bank transfer currencies available for this deposit.
    #[serde(default)]
    pub currencies: Vec<CreateDepositsResponseMethodsBankCurrenciesItem>,
}

impl CreateDepositsResponseMethodsBank {
    pub fn builder() -> CreateDepositsResponseMethodsBankBuilder {
        <CreateDepositsResponseMethodsBankBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDepositsResponseMethodsBankBuilder {
    currencies: Option<Vec<CreateDepositsResponseMethodsBankCurrenciesItem>>,
}

impl CreateDepositsResponseMethodsBankBuilder {
    pub fn currencies(
        mut self,
        value: Vec<CreateDepositsResponseMethodsBankCurrenciesItem>,
    ) -> Self {
        self.currencies = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateDepositsResponseMethodsBank`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currencies`](CreateDepositsResponseMethodsBankBuilder::currencies)
    pub fn build(self) -> Result<CreateDepositsResponseMethodsBank, BuildError> {
        Ok(CreateDepositsResponseMethodsBank {
            currencies: self
                .currencies
                .ok_or_else(|| BuildError::missing_field("currencies"))?,
        })
    }
}
