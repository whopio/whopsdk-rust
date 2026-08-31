pub use crate::prelude::*;

/// Category `balance` only. Names one of the buyer's spendable platform balances. Requires a buyer credential — whether the caller may spend the wallet is checked against their own grants, so another user's id reads as not found.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateConfirmationTokensRequestPaymentMethodBalance {
    /// The balance to spend — a balance id (ldgr_) from GET /balances.
    #[serde(default)]
    pub id: String,
}

impl CreateConfirmationTokensRequestPaymentMethodBalance {
    pub fn builder() -> CreateConfirmationTokensRequestPaymentMethodBalanceBuilder {
        <CreateConfirmationTokensRequestPaymentMethodBalanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateConfirmationTokensRequestPaymentMethodBalanceBuilder {
    id: Option<String>,
}

impl CreateConfirmationTokensRequestPaymentMethodBalanceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateConfirmationTokensRequestPaymentMethodBalance`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateConfirmationTokensRequestPaymentMethodBalanceBuilder::id)
    pub fn build(self) -> Result<CreateConfirmationTokensRequestPaymentMethodBalance, BuildError> {
        Ok(CreateConfirmationTokensRequestPaymentMethodBalance {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
