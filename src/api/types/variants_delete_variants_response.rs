pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteVariantsResponse {
    /// Always true.
    #[serde(default)]
    pub deleted: bool,
    /// ID of the deleted variant.
    #[serde(default)]
    pub id: String,
}

impl DeleteVariantsResponse {
    pub fn builder() -> DeleteVariantsResponseBuilder {
        <DeleteVariantsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteVariantsResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeleteVariantsResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteVariantsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteVariantsResponseBuilder::deleted)
    /// - [`id`](DeleteVariantsResponseBuilder::id)
    pub fn build(self) -> Result<DeleteVariantsResponse, BuildError> {
        Ok(DeleteVariantsResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
