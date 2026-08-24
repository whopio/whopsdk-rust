pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CalculateTaxPlansRequestTaxIdsItem {
    /// Tax ID type, such as `eu_vat` for an EU VAT number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CalculateTaxPlansRequestTaxIdsItemType>,
    /// Tax ID value, for example `DE123456789`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CalculateTaxPlansRequestTaxIdsItem {
    pub fn builder() -> CalculateTaxPlansRequestTaxIdsItemBuilder {
        <CalculateTaxPlansRequestTaxIdsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalculateTaxPlansRequestTaxIdsItemBuilder {
    r#type: Option<CalculateTaxPlansRequestTaxIdsItemType>,
    value: Option<String>,
}

impl CalculateTaxPlansRequestTaxIdsItemBuilder {
    pub fn r#type(mut self, value: CalculateTaxPlansRequestTaxIdsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CalculateTaxPlansRequestTaxIdsItem`].
    pub fn build(self) -> Result<CalculateTaxPlansRequestTaxIdsItem, BuildError> {
        Ok(CalculateTaxPlansRequestTaxIdsItem {
            r#type: self.r#type,
            value: self.value,
        })
    }
}
