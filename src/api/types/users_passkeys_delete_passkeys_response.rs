pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeletePasskeysResponse {
    /// Always `true`: the passkey was removed.
    #[serde(default)]
    pub deleted: bool,
    /// The ID of the deleted passkey.
    #[serde(default)]
    pub id: String,
}

impl DeletePasskeysResponse {
    pub fn builder() -> DeletePasskeysResponseBuilder {
        <DeletePasskeysResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeletePasskeysResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeletePasskeysResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeletePasskeysResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeletePasskeysResponseBuilder::deleted)
    /// - [`id`](DeletePasskeysResponseBuilder::id)
    pub fn build(self) -> Result<DeletePasskeysResponse, BuildError> {
        Ok(DeletePasskeysResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
