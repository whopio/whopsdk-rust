pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteAppsResponse {
    /// Always true.
    #[serde(default)]
    pub deleted: bool,
    /// ID of the deleted app.
    #[serde(default)]
    pub id: String,
}

impl DeleteAppsResponse {
    pub fn builder() -> DeleteAppsResponseBuilder {
        <DeleteAppsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteAppsResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeleteAppsResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteAppsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteAppsResponseBuilder::deleted)
    /// - [`id`](DeleteAppsResponseBuilder::id)
    pub fn build(self) -> Result<DeleteAppsResponse, BuildError> {
        Ok(DeleteAppsResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
