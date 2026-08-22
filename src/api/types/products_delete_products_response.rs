pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteProductsResponse {
    /// Always true.
    #[serde(default)]
    pub deleted: bool,
    /// ID of the deleted product.
    #[serde(default)]
    pub id: String,
}

impl DeleteProductsResponse {
    pub fn builder() -> DeleteProductsResponseBuilder {
        <DeleteProductsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteProductsResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeleteProductsResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteProductsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteProductsResponseBuilder::deleted)
    /// - [`id`](DeleteProductsResponseBuilder::id)
    pub fn build(self) -> Result<DeleteProductsResponse, BuildError> {
        Ok(DeleteProductsResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
