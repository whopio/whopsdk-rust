pub use crate::prelude::*;

/// The bank account-specific details for this payment method, including bank name and last four digits.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodUsBankAccountPaymentMethodUsBankAccount {
    /// The type of bank account (e.g., checking, savings).
    #[serde(default)]
    pub account_type: String,
    /// The name of the financial institution holding the account.
    #[serde(default)]
    pub bank_name: String,
    /// The last four digits of the bank account number.
    #[serde(default)]
    pub last4: String,
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
    /// This method will fail if any of the following fields are not set:
    /// - [`account_type`](PaymentMethodUsBankAccountPaymentMethodUsBankAccountBuilder::account_type)
    /// - [`bank_name`](PaymentMethodUsBankAccountPaymentMethodUsBankAccountBuilder::bank_name)
    /// - [`last4`](PaymentMethodUsBankAccountPaymentMethodUsBankAccountBuilder::last4)
    pub fn build(self) -> Result<PaymentMethodUsBankAccountPaymentMethodUsBankAccount, BuildError> {
        Ok(PaymentMethodUsBankAccountPaymentMethodUsBankAccount {
            account_type: self
                .account_type
                .ok_or_else(|| BuildError::missing_field("account_type"))?,
            bank_name: self
                .bank_name
                .ok_or_else(|| BuildError::missing_field("bank_name"))?,
            last4: self
                .last4
                .ok_or_else(|| BuildError::missing_field("last4"))?,
        })
    }
}
