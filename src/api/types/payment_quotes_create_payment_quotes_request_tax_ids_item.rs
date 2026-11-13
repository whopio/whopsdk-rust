pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreatePaymentQuotesRequestTaxIdsItem {
    /// The registration type. See the [tax IDs guide](/developer/guides/tax-ids) for an example of each type.
    pub r#type: CreatePaymentQuotesRequestTaxIdsItemType,
    /// The registration number as issued. An `eu_vat` number must start with its two-letter country code (Greece uses `EL`). See the [tax IDs guide](/developer/guides/tax-ids).
    #[serde(default)]
    pub value: String,
}

impl CreatePaymentQuotesRequestTaxIdsItem {
    pub fn builder() -> CreatePaymentQuotesRequestTaxIdsItemBuilder {
        <CreatePaymentQuotesRequestTaxIdsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentQuotesRequestTaxIdsItemBuilder {
    r#type: Option<CreatePaymentQuotesRequestTaxIdsItemType>,
    value: Option<String>,
}

impl CreatePaymentQuotesRequestTaxIdsItemBuilder {
    pub fn r#type(mut self, value: CreatePaymentQuotesRequestTaxIdsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentQuotesRequestTaxIdsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](CreatePaymentQuotesRequestTaxIdsItemBuilder::r#type)
    /// - [`value`](CreatePaymentQuotesRequestTaxIdsItemBuilder::value)
    pub fn build(self) -> Result<CreatePaymentQuotesRequestTaxIdsItem, BuildError> {
        Ok(CreatePaymentQuotesRequestTaxIdsItem {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
