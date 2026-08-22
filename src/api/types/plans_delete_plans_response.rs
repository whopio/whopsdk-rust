pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeletePlansResponse {
    /// Always true.
    #[serde(default)]
    pub deleted: bool,
    /// ID of the deleted plan.
    #[serde(default)]
    pub id: String,
}

impl DeletePlansResponse {
    pub fn builder() -> DeletePlansResponseBuilder {
        <DeletePlansResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeletePlansResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeletePlansResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeletePlansResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeletePlansResponseBuilder::deleted)
    /// - [`id`](DeletePlansResponseBuilder::id)
    pub fn build(self) -> Result<DeletePlansResponse, BuildError> {
        Ok(DeletePlansResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
