pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LedgerActivityResourceBankBank {
    /// Bank account holder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// Bank account type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Bank name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// Last four digits of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
}

impl LedgerActivityResourceBankBank {
    pub fn builder() -> LedgerActivityResourceBankBankBuilder {
        <LedgerActivityResourceBankBankBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerActivityResourceBankBankBuilder {
    account_name: Option<String>,
    account_type: Option<String>,
    bank_name: Option<String>,
    last4: Option<String>,
}

impl LedgerActivityResourceBankBankBuilder {
    pub fn account_name(mut self, value: impl Into<String>) -> Self {
        self.account_name = Some(value.into());
        self
    }

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

    /// Consumes the builder and constructs a [`LedgerActivityResourceBankBank`].
    pub fn build(self) -> Result<LedgerActivityResourceBankBank, BuildError> {
        Ok(LedgerActivityResourceBankBank {
            account_name: self.account_name,
            account_type: self.account_type,
            bank_name: self.bank_name,
            last4: self.last4,
        })
    }
}
