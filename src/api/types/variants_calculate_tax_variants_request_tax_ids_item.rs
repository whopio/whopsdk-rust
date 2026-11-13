pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CalculateTaxVariantsRequestTaxIdsItem {
    /// Tax ID type, such as `eu_vat` for an EU VAT number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CalculateTaxVariantsRequestTaxIdsItemType>,
    /// Tax ID value, for example `DE123456789`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CalculateTaxVariantsRequestTaxIdsItem {
    pub fn builder() -> CalculateTaxVariantsRequestTaxIdsItemBuilder {
        <CalculateTaxVariantsRequestTaxIdsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalculateTaxVariantsRequestTaxIdsItemBuilder {
    r#type: Option<CalculateTaxVariantsRequestTaxIdsItemType>,
    value: Option<String>,
}

impl CalculateTaxVariantsRequestTaxIdsItemBuilder {
    pub fn r#type(mut self, value: CalculateTaxVariantsRequestTaxIdsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CalculateTaxVariantsRequestTaxIdsItem`].
    pub fn build(self) -> Result<CalculateTaxVariantsRequestTaxIdsItem, BuildError> {
        Ok(CalculateTaxVariantsRequestTaxIdsItem {
            r#type: self.r#type,
            value: self.value,
        })
    }
}
