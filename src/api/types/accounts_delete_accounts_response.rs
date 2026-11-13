pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteAccountsResponse {
    /// Always true.
    #[serde(default)]
    pub deleted: bool,
    /// ID of the deleted subaccount.
    #[serde(default)]
    pub id: String,
}

impl DeleteAccountsResponse {
    pub fn builder() -> DeleteAccountsResponseBuilder {
        <DeleteAccountsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteAccountsResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeleteAccountsResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteAccountsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteAccountsResponseBuilder::deleted)
    /// - [`id`](DeleteAccountsResponseBuilder::id)
    pub fn build(self) -> Result<DeleteAccountsResponse, BuildError> {
        Ok(DeleteAccountsResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
