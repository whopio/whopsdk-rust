pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LedgerActivityResourceAccountReference {
    /// Masked account reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_reference: Option<String>,
    /// Destination currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_currency_code: Option<String>,
    /// Payout method ID.
    #[serde(default)]
    pub id: String,
    /// Payout institution name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<String>,
    /// Payout method nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    pub object: LedgerActivityResourceAccountReferenceObject,
    /// Payout provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}

impl LedgerActivityResourceAccountReference {
    pub fn builder() -> LedgerActivityResourceAccountReferenceBuilder {
        <LedgerActivityResourceAccountReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerActivityResourceAccountReferenceBuilder {
    account_reference: Option<String>,
    destination_currency_code: Option<String>,
    id: Option<String>,
    institution_name: Option<String>,
    nickname: Option<String>,
    object: Option<LedgerActivityResourceAccountReferenceObject>,
    provider: Option<String>,
}

impl LedgerActivityResourceAccountReferenceBuilder {
    pub fn account_reference(mut self, value: impl Into<String>) -> Self {
        self.account_reference = Some(value.into());
        self
    }

    pub fn destination_currency_code(mut self, value: impl Into<String>) -> Self {
        self.destination_currency_code = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn institution_name(mut self, value: impl Into<String>) -> Self {
        self.institution_name = Some(value.into());
        self
    }

    pub fn nickname(mut self, value: impl Into<String>) -> Self {
        self.nickname = Some(value.into());
        self
    }

    pub fn object(mut self, value: LedgerActivityResourceAccountReferenceObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LedgerActivityResourceAccountReference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LedgerActivityResourceAccountReferenceBuilder::id)
    /// - [`object`](LedgerActivityResourceAccountReferenceBuilder::object)
    pub fn build(self) -> Result<LedgerActivityResourceAccountReference, BuildError> {
        Ok(LedgerActivityResourceAccountReference {
            account_reference: self.account_reference,
            destination_currency_code: self.destination_currency_code,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            institution_name: self.institution_name,
            nickname: self.nickname,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            provider: self.provider,
        })
    }
}
