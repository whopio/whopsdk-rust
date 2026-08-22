pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteWebhooksResponse {
    /// Always `true`: the resource was deleted.
    #[serde(default)]
    pub deleted: bool,
    /// The ID of the deleted resource.
    #[serde(default)]
    pub id: String,
}

impl DeleteWebhooksResponse {
    pub fn builder() -> DeleteWebhooksResponseBuilder {
        <DeleteWebhooksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteWebhooksResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeleteWebhooksResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteWebhooksResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteWebhooksResponseBuilder::deleted)
    /// - [`id`](DeleteWebhooksResponseBuilder::id)
    pub fn build(self) -> Result<DeleteWebhooksResponse, BuildError> {
        Ok(DeleteWebhooksResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
