pub use crate::prelude::*;

/// Category `bank_debit` only. A type that declares a secure field (`sepa_debit`) sends the element's tokenized credential as `token`. `us_bank_account` sends nothing here — the buyer links the account after confirm, through the hosted bank-connection flow the payment parks behind.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateConfirmationTokensRequestPaymentMethodBankDebit {
    /// The Basis Theory token the element vaulted the account details into. Required for types declaring a secure field; rejected for types that collect after confirm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl CreateConfirmationTokensRequestPaymentMethodBankDebit {
    pub fn builder() -> CreateConfirmationTokensRequestPaymentMethodBankDebitBuilder {
        <CreateConfirmationTokensRequestPaymentMethodBankDebitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateConfirmationTokensRequestPaymentMethodBankDebitBuilder {
    token: Option<String>,
}

impl CreateConfirmationTokensRequestPaymentMethodBankDebitBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateConfirmationTokensRequestPaymentMethodBankDebit`].
    pub fn build(
        self,
    ) -> Result<CreateConfirmationTokensRequestPaymentMethodBankDebit, BuildError> {
        Ok(CreateConfirmationTokensRequestPaymentMethodBankDebit { token: self.token })
    }
}
