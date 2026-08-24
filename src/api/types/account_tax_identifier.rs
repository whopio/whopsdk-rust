pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AccountTaxIdentifier {
    /// Tax identifier ID.
    #[serde(default)]
    pub id: String,
    /// Tax ID type.
    pub tax_id_type: AccountTaxIdentifierTaxIdType,
    /// Tax ID value.
    #[serde(default)]
    pub tax_id_value: String,
}

impl AccountTaxIdentifier {
    pub fn builder() -> AccountTaxIdentifierBuilder {
        <AccountTaxIdentifierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountTaxIdentifierBuilder {
    id: Option<String>,
    tax_id_type: Option<AccountTaxIdentifierTaxIdType>,
    tax_id_value: Option<String>,
}

impl AccountTaxIdentifierBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn tax_id_type(mut self, value: AccountTaxIdentifierTaxIdType) -> Self {
        self.tax_id_type = Some(value);
        self
    }

    pub fn tax_id_value(mut self, value: impl Into<String>) -> Self {
        self.tax_id_value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountTaxIdentifier`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AccountTaxIdentifierBuilder::id)
    /// - [`tax_id_type`](AccountTaxIdentifierBuilder::tax_id_type)
    /// - [`tax_id_value`](AccountTaxIdentifierBuilder::tax_id_value)
    pub fn build(self) -> Result<AccountTaxIdentifier, BuildError> {
        Ok(AccountTaxIdentifier {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            tax_id_type: self
                .tax_id_type
                .ok_or_else(|| BuildError::missing_field("tax_id_type"))?,
            tax_id_value: self
                .tax_id_value
                .ok_or_else(|| BuildError::missing_field("tax_id_value"))?,
        })
    }
}
