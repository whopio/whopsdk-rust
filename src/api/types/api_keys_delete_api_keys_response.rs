pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteApiKeysResponse {
    /// Always `true`: the key was revoked.
    #[serde(default)]
    pub deleted: bool,
    /// The ID of the revoked key.
    #[serde(default)]
    pub id: String,
}

impl DeleteApiKeysResponse {
    pub fn builder() -> DeleteApiKeysResponseBuilder {
        <DeleteApiKeysResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteApiKeysResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeleteApiKeysResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteApiKeysResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteApiKeysResponseBuilder::deleted)
    /// - [`id`](DeleteApiKeysResponseBuilder::id)
    pub fn build(self) -> Result<DeleteApiKeysResponse, BuildError> {
        Ok(DeleteApiKeysResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
