pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteAdGroupsResponse {
    /// Always true.
    #[serde(default)]
    pub deleted: bool,
    /// ID of the deleted ad group.
    #[serde(default)]
    pub id: String,
}

impl DeleteAdGroupsResponse {
    pub fn builder() -> DeleteAdGroupsResponseBuilder {
        <DeleteAdGroupsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteAdGroupsResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeleteAdGroupsResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteAdGroupsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteAdGroupsResponseBuilder::deleted)
    /// - [`id`](DeleteAdGroupsResponseBuilder::id)
    pub fn build(self) -> Result<DeleteAdGroupsResponse, BuildError> {
        Ok(DeleteAdGroupsResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
