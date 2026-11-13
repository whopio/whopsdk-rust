pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteAdPixelsResponse {
    #[serde(default)]
    pub deleted: bool,
    #[serde(default)]
    pub id: String,
}

impl DeleteAdPixelsResponse {
    pub fn builder() -> DeleteAdPixelsResponseBuilder {
        <DeleteAdPixelsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteAdPixelsResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeleteAdPixelsResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteAdPixelsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteAdPixelsResponseBuilder::deleted)
    /// - [`id`](DeleteAdPixelsResponseBuilder::id)
    pub fn build(self) -> Result<DeleteAdPixelsResponse, BuildError> {
        Ok(DeleteAdPixelsResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
