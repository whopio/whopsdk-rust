pub use crate::prelude::*;

/// The collected method: `type` names the payment method, `category` names the payload shape, and the category-keyed object carries the payload. Wallets are the exception: their payload rides the type key (`apple_pay` / `google_pay`). Send exactly the one payload arm the category selects — extra arms are rejected. Redirect-flow methods (category `redirect`, `bank_transfer`, `voucher`, and redirect wallets like `cashapp`) collect nothing and send no payload arm.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateConfirmationTokensRequestPaymentMethod {
    /// Type `apple_pay` (category `wallet`) only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<CreateConfirmationTokensRequestPaymentMethodApplePay>,
    /// Category `balance` only. Names one of the buyer's spendable platform balances. Requires a buyer credential — whether the caller may spend the wallet is checked against their own grants, so another user's id reads as not found.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<CreateConfirmationTokensRequestPaymentMethodBalance>,
    /// Category `bank_debit` only. A type that declares a secure field (`sepa_debit`) sends the element's tokenized credential as `token`. `us_bank_account` sends nothing here — the buyer links the account after confirm, through the hosted bank-connection flow the payment parks behind.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_debit: Option<CreateConfirmationTokensRequestPaymentMethodBankDebit>,
    /// Category `card` only. Exactly one of `token` or `token_intent`; display fields ride alongside.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateConfirmationTokensRequestPaymentMethodCard>,
    /// The payload shape the surface collected. Must be the category the type resolves to — it is derived server-side and a mismatch is rejected. `saved` and `balance` are the exceptions: they name a method already on file or a spendable balance rather than one collected here.
    pub category: CreateConfirmationTokensRequestPaymentMethodCategory,
    /// Type `google_pay` (category `wallet`) only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<CreateConfirmationTokensRequestPaymentMethodGooglePay>,
    /// The buyer's identity document when the charge currency has a payer_document_requirements entry for this method, such as ARS card, MODO, or Rapipago. This is independent of the method category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_document: Option<CreateConfirmationTokensRequestPaymentMethodPayerDocument>,
    /// Category `saved` only. Names one of the buyer's own stored payment methods. Requires a buyer credential — the wallet read is scoped to that account, so another user's id reads as not found.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved: Option<CreateConfirmationTokensRequestPaymentMethodSaved>,
    /// The payment method type, for example `card` or `ideal`. Required for every category except `saved` and `balance`, where it is read from the referenced method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl CreateConfirmationTokensRequestPaymentMethod {
    pub fn builder() -> CreateConfirmationTokensRequestPaymentMethodBuilder {
        <CreateConfirmationTokensRequestPaymentMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateConfirmationTokensRequestPaymentMethodBuilder {
    apple_pay: Option<CreateConfirmationTokensRequestPaymentMethodApplePay>,
    balance: Option<CreateConfirmationTokensRequestPaymentMethodBalance>,
    bank_debit: Option<CreateConfirmationTokensRequestPaymentMethodBankDebit>,
    card: Option<CreateConfirmationTokensRequestPaymentMethodCard>,
    category: Option<CreateConfirmationTokensRequestPaymentMethodCategory>,
    google_pay: Option<CreateConfirmationTokensRequestPaymentMethodGooglePay>,
    payer_document: Option<CreateConfirmationTokensRequestPaymentMethodPayerDocument>,
    saved: Option<CreateConfirmationTokensRequestPaymentMethodSaved>,
    r#type: Option<String>,
}

impl CreateConfirmationTokensRequestPaymentMethodBuilder {
    pub fn apple_pay(
        mut self,
        value: CreateConfirmationTokensRequestPaymentMethodApplePay,
    ) -> Self {
        self.apple_pay = Some(value);
        self
    }

    pub fn balance(mut self, value: CreateConfirmationTokensRequestPaymentMethodBalance) -> Self {
        self.balance = Some(value);
        self
    }

    pub fn bank_debit(
        mut self,
        value: CreateConfirmationTokensRequestPaymentMethodBankDebit,
    ) -> Self {
        self.bank_debit = Some(value);
        self
    }

    pub fn card(mut self, value: CreateConfirmationTokensRequestPaymentMethodCard) -> Self {
        self.card = Some(value);
        self
    }

    pub fn category(mut self, value: CreateConfirmationTokensRequestPaymentMethodCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn google_pay(
        mut self,
        value: CreateConfirmationTokensRequestPaymentMethodGooglePay,
    ) -> Self {
        self.google_pay = Some(value);
        self
    }

    pub fn payer_document(
        mut self,
        value: CreateConfirmationTokensRequestPaymentMethodPayerDocument,
    ) -> Self {
        self.payer_document = Some(value);
        self
    }

    pub fn saved(mut self, value: CreateConfirmationTokensRequestPaymentMethodSaved) -> Self {
        self.saved = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateConfirmationTokensRequestPaymentMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](CreateConfirmationTokensRequestPaymentMethodBuilder::category)
    pub fn build(self) -> Result<CreateConfirmationTokensRequestPaymentMethod, BuildError> {
        Ok(CreateConfirmationTokensRequestPaymentMethod {
            apple_pay: self.apple_pay,
            balance: self.balance,
            bank_debit: self.bank_debit,
            card: self.card,
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            google_pay: self.google_pay,
            payer_document: self.payer_document,
            saved: self.saved,
            r#type: self.r#type,
        })
    }
}
