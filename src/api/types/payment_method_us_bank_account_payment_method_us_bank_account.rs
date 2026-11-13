pub use crate::prelude::*;

/// The bank account-specific details for this payment method, including bank name and last four digits.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodUsBankAccountPaymentMethodUsBankAccount {
    /// The type of bank account (e.g., checking, savings).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// The name of the financial institution holding the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// The last four digits of the bank account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
}

impl PaymentMethodUsBankAccountPaymentMethodUsBankAccount {
    pub fn builder() -> PaymentMethodUsBankAccountPaymentMethodUsBankAccountBuilder {
        <PaymentMethodUsBankAccountPaymentMethodUsBankAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodUsBankAccountPaymentMethodUsBankAccountBuilder {
    account_type: Option<String>,
    bank_name: Option<String>,
    last4: Option<String>,
}

impl PaymentMethodUsBankAccountPaymentMethodUsBankAccountBuilder {
    pub fn account_type(mut self, value: impl Into<String>) -> Self {
        self.account_type = Some(value.into());
        self
    }

    pub fn bank_name(mut self, value: impl Into<String>) -> Self {
        self.bank_name = Some(value.into());
        self
    }

    pub fn last4(mut self, value: impl Into<String>) -> Self {
        self.last4 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodUsBankAccountPaymentMethodUsBankAccount`].
    pub fn build(self) -> Result<PaymentMethodUsBankAccountPaymentMethodUsBankAccount, BuildError> {
        Ok(PaymentMethodUsBankAccountPaymentMethodUsBankAccount {
            account_type: self.account_type,
            bank_name: self.bank_name,
            last4: self.last4,
        })
    }
}
