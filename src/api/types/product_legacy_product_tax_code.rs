pub use crate::prelude::*;

/// The tax classification code applied to purchases of this product for sales tax calculation. Null if no tax code is assigned.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProductLegacyProductTaxCode {
    /// The unique identifier for the product tax code.
    #[serde(default)]
    pub id: String,
    /// Human-readable name of this tax classification, such as 'Digital - SaaS'.
    #[serde(default)]
    pub name: String,
    /// Broad product category this tax code covers, such as physical goods or digital services.
    pub product_type: ProductTaxCodeProductTypes,
}

impl ProductLegacyProductTaxCode {
    pub fn builder() -> ProductLegacyProductTaxCodeBuilder {
        <ProductLegacyProductTaxCodeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductLegacyProductTaxCodeBuilder {
    id: Option<String>,
    name: Option<String>,
    product_type: Option<ProductTaxCodeProductTypes>,
}

impl ProductLegacyProductTaxCodeBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn product_type(mut self, value: ProductTaxCodeProductTypes) -> Self {
        self.product_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProductLegacyProductTaxCode`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ProductLegacyProductTaxCodeBuilder::id)
    /// - [`name`](ProductLegacyProductTaxCodeBuilder::name)
    /// - [`product_type`](ProductLegacyProductTaxCodeBuilder::product_type)
    pub fn build(self) -> Result<ProductLegacyProductTaxCode, BuildError> {
        Ok(ProductLegacyProductTaxCode {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            product_type: self
                .product_type
                .ok_or_else(|| BuildError::missing_field("product_type"))?,
        })
    }
}
