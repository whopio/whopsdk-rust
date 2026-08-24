pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LedgerActivityResourceBank {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<LedgerActivityResourceBankBank>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<LedgerActivityResourceBankCard>,
    /// Email identifier for email-based payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_identifier: Option<String>,
    /// Payment gateway type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_type: Option<String>,
    /// Payment method ID.
    #[serde(default)]
    pub id: String,
    pub object: LedgerActivityResourceBankObject,
    /// Payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
}

impl LedgerActivityResourceBank {
    pub fn builder() -> LedgerActivityResourceBankBuilder {
        <LedgerActivityResourceBankBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerActivityResourceBankBuilder {
    bank: Option<LedgerActivityResourceBankBank>,
    card: Option<LedgerActivityResourceBankCard>,
    email_identifier: Option<String>,
    gateway_type: Option<String>,
    id: Option<String>,
    object: Option<LedgerActivityResourceBankObject>,
    payment_method_type: Option<String>,
}

impl LedgerActivityResourceBankBuilder {
    pub fn bank(mut self, value: LedgerActivityResourceBankBank) -> Self {
        self.bank = Some(value);
        self
    }

    pub fn card(mut self, value: LedgerActivityResourceBankCard) -> Self {
        self.card = Some(value);
        self
    }

    pub fn email_identifier(mut self, value: impl Into<String>) -> Self {
        self.email_identifier = Some(value.into());
        self
    }

    pub fn gateway_type(mut self, value: impl Into<String>) -> Self {
        self.gateway_type = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: LedgerActivityResourceBankObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn payment_method_type(mut self, value: impl Into<String>) -> Self {
        self.payment_method_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LedgerActivityResourceBank`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LedgerActivityResourceBankBuilder::id)
    /// - [`object`](LedgerActivityResourceBankBuilder::object)
    pub fn build(self) -> Result<LedgerActivityResourceBank, BuildError> {
        Ok(LedgerActivityResourceBank {
            bank: self.bank,
            card: self.card,
            email_identifier: self.email_identifier,
            gateway_type: self.gateway_type,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            payment_method_type: self.payment_method_type,
        })
    }
}
