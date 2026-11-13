pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteInvoicesResponse {
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
    /// The ID of the deleted invoice, prefixed `pay_`.
    #[serde(default)]
    pub id: String,
}

impl DeleteInvoicesResponse {
    pub fn builder() -> DeleteInvoicesResponseBuilder {
        <DeleteInvoicesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteInvoicesResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeleteInvoicesResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteInvoicesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteInvoicesResponseBuilder::deleted)
    /// - [`id`](DeleteInvoicesResponseBuilder::id)
    pub fn build(self) -> Result<DeleteInvoicesResponse, BuildError> {
        Ok(DeleteInvoicesResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
