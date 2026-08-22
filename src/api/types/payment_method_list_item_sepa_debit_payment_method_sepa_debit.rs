pub use crate::prelude::*;

/// The SEPA Direct Debit-specific details for this payment method, including bank code and last four IBAN digits.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemSepaDebitPaymentMethodSepaDebit {
    /// The bank code of the financial institution associated with this SEPA account. Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    /// The branch code of the financial institution associated with this SEPA account. Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_code: Option<String>,
    /// The two-letter ISO country code where the bank account is located. Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The last four digits of the IBAN associated with this SEPA account. Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
}

impl PaymentMethodListItemSepaDebitPaymentMethodSepaDebit {
    pub fn builder() -> PaymentMethodListItemSepaDebitPaymentMethodSepaDebitBuilder {
        <PaymentMethodListItemSepaDebitPaymentMethodSepaDebitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemSepaDebitPaymentMethodSepaDebitBuilder {
    bank_code: Option<String>,
    branch_code: Option<String>,
    country: Option<String>,
    last4: Option<String>,
}

impl PaymentMethodListItemSepaDebitPaymentMethodSepaDebitBuilder {
    pub fn bank_code(mut self, value: impl Into<String>) -> Self {
        self.bank_code = Some(value.into());
        self
    }

    pub fn branch_code(mut self, value: impl Into<String>) -> Self {
        self.branch_code = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn last4(mut self, value: impl Into<String>) -> Self {
        self.last4 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemSepaDebitPaymentMethodSepaDebit`].
    pub fn build(self) -> Result<PaymentMethodListItemSepaDebitPaymentMethodSepaDebit, BuildError> {
        Ok(PaymentMethodListItemSepaDebitPaymentMethodSepaDebit {
            bank_code: self.bank_code,
            branch_code: self.branch_code,
            country: self.country,
            last4: self.last4,
        })
    }
}
