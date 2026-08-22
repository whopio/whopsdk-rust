pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeletePromoCodesResponse {
    #[serde(default)]
    pub deleted: bool,
    #[serde(default)]
    pub id: String,
}

impl DeletePromoCodesResponse {
    pub fn builder() -> DeletePromoCodesResponseBuilder {
        <DeletePromoCodesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeletePromoCodesResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeletePromoCodesResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeletePromoCodesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeletePromoCodesResponseBuilder::deleted)
    /// - [`id`](DeletePromoCodesResponseBuilder::id)
    pub fn build(self) -> Result<DeletePromoCodesResponse, BuildError> {
        Ok(DeletePromoCodesResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
