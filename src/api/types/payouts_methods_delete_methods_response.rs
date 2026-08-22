pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteMethodsResponse {
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
    /// ID of the deleted payout method, prefixed `potk_`.
    #[serde(default)]
    pub id: String,
}

impl DeleteMethodsResponse {
    pub fn builder() -> DeleteMethodsResponseBuilder {
        <DeleteMethodsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteMethodsResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeleteMethodsResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteMethodsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteMethodsResponseBuilder::deleted)
    /// - [`id`](DeleteMethodsResponseBuilder::id)
    pub fn build(self) -> Result<DeleteMethodsResponse, BuildError> {
        Ok(DeleteMethodsResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
