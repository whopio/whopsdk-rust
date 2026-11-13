pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateAccountsRequestTaxIdentifiersItem {
    /// Tax ID type, for example `eu_vat`, `gb_vat`, or `us_ein`. See the [tax IDs guide](/developer/guides/tax-ids) for an example of each type.
    pub tax_id_type: UpdateAccountsRequestTaxIdentifiersItemTaxIdType,
    /// Tax ID value, for example `DE123456789`. An `eu_vat` number must start with its two-letter country code (Greece uses `EL`); spaces, dots, and dashes are removed and letters are uppercased. See the [tax IDs guide](/developer/guides/tax-ids).
    #[serde(default)]
    pub tax_id_value: String,
}

impl UpdateAccountsRequestTaxIdentifiersItem {
    pub fn builder() -> UpdateAccountsRequestTaxIdentifiersItemBuilder {
        <UpdateAccountsRequestTaxIdentifiersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAccountsRequestTaxIdentifiersItemBuilder {
    tax_id_type: Option<UpdateAccountsRequestTaxIdentifiersItemTaxIdType>,
    tax_id_value: Option<String>,
}

impl UpdateAccountsRequestTaxIdentifiersItemBuilder {
    pub fn tax_id_type(mut self, value: UpdateAccountsRequestTaxIdentifiersItemTaxIdType) -> Self {
        self.tax_id_type = Some(value);
        self
    }

    pub fn tax_id_value(mut self, value: impl Into<String>) -> Self {
        self.tax_id_value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAccountsRequestTaxIdentifiersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tax_id_type`](UpdateAccountsRequestTaxIdentifiersItemBuilder::tax_id_type)
    /// - [`tax_id_value`](UpdateAccountsRequestTaxIdentifiersItemBuilder::tax_id_value)
    pub fn build(self) -> Result<UpdateAccountsRequestTaxIdentifiersItem, BuildError> {
        Ok(UpdateAccountsRequestTaxIdentifiersItem {
            tax_id_type: self
                .tax_id_type
                .ok_or_else(|| BuildError::missing_field("tax_id_type"))?,
            tax_id_value: self
                .tax_id_value
                .ok_or_else(|| BuildError::missing_field("tax_id_value"))?,
        })
    }
}
