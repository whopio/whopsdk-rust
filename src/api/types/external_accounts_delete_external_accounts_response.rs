pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteExternalAccountsResponse {
    /// Always true.
    #[serde(default)]
    pub deleted: bool,
    /// ID of the disconnected external account.
    #[serde(default)]
    pub id: String,
}

impl DeleteExternalAccountsResponse {
    pub fn builder() -> DeleteExternalAccountsResponseBuilder {
        <DeleteExternalAccountsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteExternalAccountsResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeleteExternalAccountsResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteExternalAccountsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteExternalAccountsResponseBuilder::deleted)
    /// - [`id`](DeleteExternalAccountsResponseBuilder::id)
    pub fn build(self) -> Result<DeleteExternalAccountsResponse, BuildError> {
        Ok(DeleteExternalAccountsResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
