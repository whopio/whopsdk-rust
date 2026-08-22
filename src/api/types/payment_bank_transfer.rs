pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentBankTransfer {
    /// The account to send to, in the local scheme's format — `account_number_label` says what to call it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// What to call `account_number` when showing it, in the local scheme's own terms — `CLABE` in Mexico, for example.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number_label: Option<String>,
    /// Exactly what the buyer must send, in the charged currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
    /// The kind of account receiving the transfer, such as a checking account, in the local system's own vocabulary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_type: Option<String>,
    /// The receiving branch, where the local system routes by branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_branch: Option<String>,
    /// The receiving bank's code in the local clearing system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    /// The receiving bank's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// The account holder's tax or identity document number, where the local system needs it to send.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_document: Option<String>,
    /// What kind of document `beneficiary_document` is, in the local system's own vocabulary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_document_type: Option<String>,
    /// Who the account belongs to — the name the buyer's bank may ask them to confirm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_name: Option<String>,
    /// A hosted page with the complete, printable instructions. If you would rather not render the details yourself, send the buyer here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_url: Option<String>,
    /// When these details stop being payable, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// The rail's own step-by-step payment text, when it supplies one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// The reference the buyer must attach to the transfer so it can be matched to this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// A second account number, where the rail publishes the same destination in more than one format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_account_number: Option<String>,
    /// What to call `secondary_account_number` when showing it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_account_number_label: Option<String>,
}

impl PaymentBankTransfer {
    pub fn builder() -> PaymentBankTransferBuilder {
        <PaymentBankTransferBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentBankTransferBuilder {
    account_number: Option<String>,
    account_number_label: Option<String>,
    amount: Option<Money>,
    bank_account_type: Option<String>,
    bank_branch: Option<String>,
    bank_code: Option<String>,
    bank_name: Option<String>,
    beneficiary_document: Option<String>,
    beneficiary_document_type: Option<String>,
    beneficiary_name: Option<String>,
    document_url: Option<String>,
    expires_at: Option<String>,
    instructions: Option<String>,
    reference: Option<String>,
    secondary_account_number: Option<String>,
    secondary_account_number_label: Option<String>,
}

impl PaymentBankTransferBuilder {
    pub fn account_number(mut self, value: impl Into<String>) -> Self {
        self.account_number = Some(value.into());
        self
    }

    pub fn account_number_label(mut self, value: impl Into<String>) -> Self {
        self.account_number_label = Some(value.into());
        self
    }

    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn bank_account_type(mut self, value: impl Into<String>) -> Self {
        self.bank_account_type = Some(value.into());
        self
    }

    pub fn bank_branch(mut self, value: impl Into<String>) -> Self {
        self.bank_branch = Some(value.into());
        self
    }

    pub fn bank_code(mut self, value: impl Into<String>) -> Self {
        self.bank_code = Some(value.into());
        self
    }

    pub fn bank_name(mut self, value: impl Into<String>) -> Self {
        self.bank_name = Some(value.into());
        self
    }

    pub fn beneficiary_document(mut self, value: impl Into<String>) -> Self {
        self.beneficiary_document = Some(value.into());
        self
    }

    pub fn beneficiary_document_type(mut self, value: impl Into<String>) -> Self {
        self.beneficiary_document_type = Some(value.into());
        self
    }

    pub fn beneficiary_name(mut self, value: impl Into<String>) -> Self {
        self.beneficiary_name = Some(value.into());
        self
    }

    pub fn document_url(mut self, value: impl Into<String>) -> Self {
        self.document_url = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn secondary_account_number(mut self, value: impl Into<String>) -> Self {
        self.secondary_account_number = Some(value.into());
        self
    }

    pub fn secondary_account_number_label(mut self, value: impl Into<String>) -> Self {
        self.secondary_account_number_label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentBankTransfer`].
    pub fn build(self) -> Result<PaymentBankTransfer, BuildError> {
        Ok(PaymentBankTransfer {
            account_number: self.account_number,
            account_number_label: self.account_number_label,
            amount: self.amount,
            bank_account_type: self.bank_account_type,
            bank_branch: self.bank_branch,
            bank_code: self.bank_code,
            bank_name: self.bank_name,
            beneficiary_document: self.beneficiary_document,
            beneficiary_document_type: self.beneficiary_document_type,
            beneficiary_name: self.beneficiary_name,
            document_url: self.document_url,
            expires_at: self.expires_at,
            instructions: self.instructions,
            reference: self.reference,
            secondary_account_number: self.secondary_account_number,
            secondary_account_number_label: self.secondary_account_number_label,
        })
    }
}
